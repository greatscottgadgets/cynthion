pipeline {
    agent any
    stages {
        stage('Build Docker Images') {
            steps {
                sh 'docker build -t cynthion https://github.com/grvvy/cynthion.git#ppps_update'
                sh 'docker build -t cynthion-test https://github.com/grvvy/cynthion-test.git#ppps_update'
            }
        }
        stage('Cynthion selftest') {
            agent{
                docker {
                    image 'cynthion'
                    reuseNode true
                    args '--name cynthion_container --group-add=46 --device-cgroup-rule="c 189:* rmw" --device /dev/bus/usb'
                }
            }
            steps {
                sh './ci/build.sh'
                sh 'hubs all off'
                retry(3) {
                    sh './ci/test.sh'
                }
                sh 'hubs all reset'
            }
        }
        stage('Cynthion-test') {
            agent{
                docker {
                    image 'cynthion-test'
                    reuseNode true
                    args '''
                            --name cynthion-test_container
                            --group-add=20
                            --group-add=46
                            --device-cgroup-rule="c 166:* rmw"
                            --device-cgroup-rule="c 189:* rmw"
                            --device /dev/bus/usb
                            --volume /run/udev/control:/run/udev/control
                            --net=host
                        '''
                }
            }
            steps {
                sh '''#!/bin/bash
                    git clone https://github.com/grvvy/cynthion-test
                    cd cynthion-test/
                    cp /tmp/calibration.dat calibration.dat
                    make
                    environment/bin/pip install --upgrade dependencies/cynthion/cynthion/python/.
                    make analyzer.bit
                '''
                sh 'hubs all off'
                retry(3) {
                    sh '''#!/bin/bash
                        hubs cyntest_tycho cyntest_greatfet cyntest_bmp reset
                        cd cynthion-test/
                        make unattended
                    '''
                }
                sh 'hubs all reset'
            }
        }
    }
    post {
        always {
            cleanWs(cleanWhenNotBuilt: false,
                    deleteDirs: true,
                    disableDeferredWipeout: true,
                    notFailBuild: true)
        }
    }
}

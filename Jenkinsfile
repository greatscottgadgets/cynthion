import org.jenkinsci.plugins.workflow.steps.FlowInterruptedException

pipeline {
    options {
        skipDefaultCheckout true
    }
    agent any
    stages {
        stage('Build Docker Image') {
            steps {
                sh 'docker build -t cynthion-test https://github.com/greatscottgadgets/cynthion-test.git'
            }
        }
        stage('Checkout as submodule') {
            steps {
                dir('cynthion-test') {
                    git url: 'https://github.com/greatscottgadgets/cynthion-test.git', branch: 'main'
                    sh 'make submodule-checkout'
                    sh 'rm -rf dependencies/cynthion'
                    dir('dependencies/cynthion') {
                        checkout scm // override pinned submodule version with current version
                    }
                }
            }
        }
        stage('Build') {
            agent{
                docker {
                    image 'cynthion-test'
                    reuseNode true
                    args '--name cynthion-test_container'
                }
            }
            steps {
                dir('cynthion-test') {
                    sh 'cp /tmp/calibration.dat calibration.dat'
                    sh 'make analyzer.bit'
                }
            }
        }
        stage('HIL Test') {
            agent {
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
                            -v /tmp/req_pipe:/tmp/req_pipe
                            -v /tmp/res_pipe:/tmp/res_pipe
                        '''
                }
            }
            steps {
                dir('cynthion-test') {
                    retry(3) {
                        script {
                            try {
                                // Allow 20 seconds for the USB hub port power server to respond
                                timeout(time: 20, unit: 'SECONDS') {
                                    sh 'hubs all off'
                                    sh 'hubs cyntest_tycho cyntest_greatfet cyntest_bmp reset'
                                }
                            }  catch (FlowInterruptedException err) {
                                // Check if the cause was specifically an exceeded timeout
                                def cause = err.getCauses().get(0)
                                if (cause instanceof org.jenkinsci.plugins.workflow.steps.TimeoutStepExecution.ExceededTimeout) {
                                    echo "USB hub port power server command timeout reached."
                                    throw err // Re-throw the exception to fail the build
                                } else {
                                    echo "Build interrupted for another reason."
                                    throw err // Re-throw the exception to fail the build
                                }
                            } catch (Exception err) {
                                echo "An unrelated error occurred: ${err.getMessage()}"
                                throw err
                            }
                            sh 'make unattended'
                        }
                    }
                }
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

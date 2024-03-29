pipeline {
    agent {
        dockerfile {
            additionalBuildArgs '--build-arg CACHEBUST=$(date +%s)'
            args '--group-add=46 --device-cgroup-rule="c 189:* rmw" -v /dev/bus/usb:/dev/bus/usb'
        }
    }
    stages {
        stage('Build') {
            steps {
                sh './ci/build.sh'
            }
        }
        stage('Test') {
            steps {
                sh './ci/configure-hubs.sh --off'
                retry(3) {
                    sh './ci/test.sh'
                }
            }
        }
    }
    post {
        always {
            sh './ci/configure-hubs.sh --reset'
            cleanWs(cleanWhenNotBuilt: false,
                    deleteDirs: true,
                    disableDeferredWipeout: true,
                    notFailBuild: true)
        }
    }
}

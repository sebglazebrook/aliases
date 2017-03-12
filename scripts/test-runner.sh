#!/bin/bash

dir="$( cd "$( dirname "${BASH_SOURCE[0]}"  )" && cd .. && pwd )"

# make sure image is built
cd $dir && docker build -t alias-test-runner -f spec/Dockerfile .

docker run -ti -e "APP_ROOT=${dir}" -v ${dir}:/code  -v /var/run/docker.sock:/var/run/docker.sock alias-test-runner bundle exec rspec spec

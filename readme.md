# Target Environment Driven Development demo code

This repo demonstrates conditional compilation using cargo feature flags. The pipeline in drone passes feature flags as Docker `--build-arg`s and then to the target environment using an Ansible Playbook. 


## Deployment

Ansible playbook lives here `./.deploy` and the drone file uses jsonnet `.drone.jsonnet` as a data template for defining a pipeline per environment. 


## Useful commands when run outside drone

    docker build -t pig --build-arg FLAGS="--features pig" .
    
    docker run -t pig
    docker tag pig 192.168.10.100:5000/pig:latest
    docker push 192.168.10.100:5000/pig:latest

    ansible-playbook --inventory .deploy/inventory.yaml \
        --extra-vars container_name=pig --extra-vars image=pig:latest \
        --extra-vars registry=192.168.10.100:5000 --extra-vars network=host \
        --private-key ~/.ssh/id_rsa_drone --user $USER \
        .deploy/playbook.yaml


## Branches

For the demo there are 5 branches, each builds on top of the previous one

- `1example` - initial example
- `2pig` - add a second implementation of the `Animal` trait
- `3conditional` - add conditional compile flags for `Sheep` and `Pig` 
- `4refactor` - create modules out of `Sheep` and `Pig` to clean up `cfg_attr` usage
- `master` - the fluff
local test(flags, image) = {
    name: 'test',
    image: image,
    pull: true,
    commands: [
        'cargo build --tests ' + flags
    ]
};

local build(name, flags, registry, proto='http') = {
    name: 'build',
    image: 'plugins/docker',
    settings: {
      repo: registry + '/' + name,
      registry: proto + '://' + registry,
      insecure: true,
      build_args: ['FLAGS=' + flags]
    },
    depends_on: ['test']
};

local deploy(name, registry) = {
    name: 'deploy',
    image: 'plugins/ansible:latest',
    settings: {
        playbook: '.deploy/playbook.yaml',
        inventory: '.deploy/inventory.yaml',
        galaxy: '.deploy/requirements.yaml',
        user: "max",
        private_key: {
            from_secret: 'ssh_private_key'
        },
        extra_vars: 'container_name=' + name + ',image=' + name + ':latest,registry=' + registry + ',network=host'
    },
    depends_on: ['build']
};

local pipeline(name, flags, image='rust:1.50', registry='192.168.10.100:5000') = {
    kind: 'pipeline',
    type: 'docker',
    name: name,
    clone: {
        depth: 1
    },
    steps: [
        test(flags, image),
        build(name, flags, registry),
        deploy(name, registry)
    ]
};

[
    pipeline('pig', '--features pig'),
    pipeline('sheep', '--features sheep --no-default-features')
]
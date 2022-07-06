docker_build(
    'agabani/tunnel-operator',
    '.',
    dockerfile='Dockerfile',
)

k8s_yaml(
    helm(
        'helm',
        name='tunnel-operator',
    )
)

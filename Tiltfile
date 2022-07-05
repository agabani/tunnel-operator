docker_build(
    'agabani/tunnel-operator',
    '.',
    dockerfile='Dockerfile',
    extra_tag=['latest']
)

k8s_yaml('.tilt/namespace.yaml')

k8s_yaml(
    helm(
        'helm',
        name='tunnel-operator',
        namespace='tunnel-operator',
        set=[
            'image.repository=agabani/tunnel-operator',
            'image.tag=latest',
        ]
    )
)

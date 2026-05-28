from setuptools import find_packages, setup

package_name = 'py_executor_example'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='ubunut2204',
    maintainer_email='ros@email.com',
    description='TODO: Package description',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'single_executor_node = py_executor_example.single_executor_node:main',
            'multi_executor_node = py_executor_example.multi_executor_node:main',
            'multi_exclusive_node = py_executor_example.multi_exclusive_node:main'
        ],
    },
)

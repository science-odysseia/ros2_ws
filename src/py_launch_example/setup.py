from setuptools import find_packages, setup
import os
from glob import glob

package_name = 'py_launch_example'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (
            os.path.join('share', package_name, 'launch'), glob('launch/*.launch.py')
        )
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
            'talker_node = py_launch_example.talker_node:main',
            'listener_node = py_launch_example.listener_node:main',
        ],
    },
)

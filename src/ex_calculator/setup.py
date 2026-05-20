from setuptools import find_packages, setup

package_name = 'ex_calculator'

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
            'argument = ex_calculator.arithmetic.argument:main',
            'operator = ex_calculator.arithmetic.operator:main',
            'calculator = ex_calculator.calculator.main:main',
            'checker = ex_calculator.checker.main:main'
        ],
    },
)

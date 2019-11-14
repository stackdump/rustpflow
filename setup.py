from setuptools import setup, find_packages

setup(
    name="pflow",
    version="0.1.0",
    author="Matthew York",
    author_email="myork@stackdump.com",
    description="",
    license='MIT',
    keywords='pflow statemachine petri-net wasm',
    packages=find_packages(),
    include_package_data=True,
    install_requires="",
    long_description="""
    rust/WASM state machine code generation from pflow xml files
    """,
    url="",
    classifiers=[
        "Development Status :: 2 - Pre-Alpha",
        "License :: OSI Approved :: MIT License"],
)

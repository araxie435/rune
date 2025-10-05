# Package manifest

## Overview

Manifest use YAML form

``` yaml
manifest:
name:
version:
description:
scopes:
dependencies:
optional-dependencies:
conflicts:
paths:
    global:
        bin:
        config:
        other:
    group:
        bin:
        config:
        other:
    user:
        bin:
        config:
        other:
scripts:
    preinstall:
    install:
    postinstall:
    preuninstall:
    uninstall:
    postuninstall:
```

List of required parameters:
- manifest
- name
- version

Other parameters are not required or have their initial values

## Parameters

### manifest

``` yaml
manifest: v1
```

Parameter for future updates. Now have only "v1" option but still required


### name

``` yaml
name: package-name
```

Name of the package. We recommend not to use capital letters.


### version

``` yaml
version: v0.0.0
```

Version of package. 

### description

``` yaml
description: The package manager
```

Short description of package

Description of packet.

### scopes
``` yaml
scopes:
    global
    group
    user
```

The scopes field defines the installation scopes supported by the package. Possible values are: global, group, and local. If nothing is set or there is no field package will allow installation in every scope

### dependencies

``` yaml
dependencies:
    package
    package2
```

Required dependencies of package. Without them package wont work at all.

### optional-dependencies

``` yaml
optional-dependencies:
    package
    package2
```

Suggested to install. Without them some functions wont work.

### conflicts

``` yaml
conflicts:
    package
    package2
```

List of incompatible programs without versions

### paths

``` yaml
paths:
    global:
        bin:
            /bin/package
        config:
            /etc/package/
        other:
            /srv/package/
    group:
        bin:
        config:
        other:
    user:
        bin:
        config:
        other:
```

Contain non-standard file paths by category. It is used for functions such as clear uninstall, upgrading with keeping old configs and other working with package insides.

### scripts

``` yaml
scripts:
    preinstall:
    install:
    postinstall:
    preuninstall:
    uninstall:
    postuninstall:
```
Using scripts you can add and override the logic of individual parts of the installation process within the established rules.

[There](scripts.md) is some more information about scripts, rules and templates for them. 
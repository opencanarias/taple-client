<div align="center">

# Tutorial for setting up a test network using scripts

<br>
    <p align="center">
    <img src= "./../../imgs/logo-taple.png" width=300px>
    </p>
<br>
</div>

## Índice
- **[🗣 Description](#-description)**
- **[💻 Pasos](#-steps)**
    - **[Running a 3-node network](#running-a-3-node-network)**
    - **[Create a governance and scheme](#create-a-governance-and-scheme)**
    - **[Create a subject](#create-a-subject)**
    - **[Create an event](#create-an-event)**
    - **[Display subjects of a node](#display-subjects-of-a-node)**
- **[📎 Anotaciones](#-annotations)**

<br />

## 🗣 Description
- In this directory, you will find a tutorial on how to set up a test network using the **TAPLE** technology through scripts.


## 💻 Steps
- Los scripts mencionados en los siguientes apartados se encuentran situados en el directorio **scripts**.
- 
### Running a 3-node network
---

#### Launching first node

First, we will run **taple-keygen**(located at TAPLE Tools), getting an output like the following:

```bash
$ ./taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e
CONTROLLER ID ED25519: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
PeerID: 12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
```
We will save these values for later. For now, let's raise a TAPLE node by executing the script **launch_node.sh**:

```bash
$ ./launch_node.sh
Requesting number: 
Enter a number to identify the node that will be deployed: 0
```
First, it will prompt for an integer. This number will be useful to determine the HTTP and P2P port that will be assigned. If any of these ports are in use, the script will detect it and complain. For our first node we will enter the number '0' as in the example.


```bash
Requesting cryptographic material.
Enter secret key: 01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e
Enter controllerID: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
```
Next, it will prompt for a **secret key** and a **controller id**. These values are the same ones we obtained previously by running **taple-keygen** and will be the ones we will enter.

```
Requesting know nodes : 
Enter address of know node (leave empty to stop requesting):
<No Address>
```
Now it's the turn for the known nodes that it will use as bootstrap nodes. Since this is our first node, we will not enter anything and will proceed by pressing ENTER. With this, we will have all the necessary parameters to run TAPLE.

```bash
[2023-01-16T12:16:23Z INFO  taple] AppSettings { network: NetworkSettings { p2p_port: 11000, addr: "/ip4/0.0.0.0/tcp", known_nodes: [] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data0" }, http_addr: "0.0.0.0", http_port: 10000, x_api_key: None, swagger_ui: false }
[2023-01-16T12:16:23Z INFO  taple] Controller ID: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
[2023-01-16T12:16:23Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-16T12:16:23Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-16T12:16:23Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-16T12:16:23Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
Copy and save those values:
NumberId :      0
ContainerID:    node-0
ControllerID :  EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
HTTP Port :     10000
PeerId :        12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
P2P Addresses :         [
"/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
"/ip4/172.17.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
"/ip4/25.33.212.160/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
"/ip4/192.168.1.46/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
]
```

#### Launching second and third node

We will repeat the previous steps. Since we are going to raise two nodes, we will run **taple-keygen** twice to obtain the cryptographic material we need:
```bash
$ ./taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015
CONTROLLER ID ED25519: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
PeerID: 12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk

$ ./taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b
CONTROLLER ID ED25519: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
PeerID: 12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S
```

In the same way, we will run **launch_node.sh** twice, following the same pattern described in the previous step, with the difference that now the address to the known node is to the first one:

```bash
...
"/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
...
```

```bash
$ ./launch_node.sh 
Requesting number.
Enter a number to identify the node that will be deployed: 1

Requesting cryptographic material.
Enter secret key: 0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015
Enter controllerID: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
Requesting know nodes.
Enter address of know node (leave empty to stop requesting):/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
Enter address of know node (leave empty to stop requesting):
<No Address>

[2023-01-16T13:01:58Z INFO  taple] AppSettings { network: NetworkSettings { p2p_port: 11001, addr: "/ip4/0.0.0.0/tcp", known_nodes: ["/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data1" }, http_addr: "0.0.0.0", http_port: 10001, x_api_key: None, swagger_ui: false }
[2023-01-16T13:01:58Z INFO  taple] Controller ID: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
[2023-01-16T13:01:58Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-16T13:01:58Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-16T13:01:58Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-16T13:01:58Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
Copy and save those values:
NumberId :      1
ContainerID:    node-1
ControllerID :  EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
HTTP Port :     10001
PeerId :        12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk
P2P Addresses :         [
"/ip4/25.33.212.160/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
"/ip4/172.17.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
"/ip4/127.0.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
"/ip4/192.168.1.46/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
]
```
```bash
$ ./launch_node.sh 
Requesting number: 
Enter a number to identify the node that will be deployed: 2

Requesting cryptographic material. 
Enter secret key: 09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b
Enter controllerID: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
Requesting know nodes.
Enter address of know node (leave empty to stop requesting):/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
Enter address of know node (leave empty to stop requesting):
<No Address>

[2023-01-16T13:03:16Z INFO  taple] AppSettings { network: NetworkSettings { p2p_port: 11002, addr: "/ip4/0.0.0.0/tcp", known_nodes: ["/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data2" }, http_addr: "0.0.0.0", http_port: 10002, x_api_key: None, swagger_ui: false }
[2023-01-16T13:03:16Z INFO  taple] Controller ID: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
[2023-01-16T13:03:16Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-16T13:03:16Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-16T13:03:16Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-16T13:03:16Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
Copy and save those values:
NumberId :      2
ContainerID:    node-2
ControllerID :  EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
HTTP Port :     10002
PeerId :        12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S
P2P Addresses :         [
"/ip4/172.17.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
"/ip4/192.168.1.46/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
"/ip4/127.0.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
"/ip4/25.33.212.160/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
]
```

We can check everything is working fine by checking we have a similar output:
```bash
docker container ps
CONTAINER ID   IMAGE          COMMAND   CREATED             STATUS             PORTS     NAMES
4554d5792bd1   taple-client   "taple"   32 minutes ago      Up 32 minutes                node-2
d634ed0975a0   taple-client   "taple"   34 minutes ago      Up 33 minutes                node-1
0680f28cfea9   taple-client   "taple"   About an hour ago   Up About an hour             node-0
```

### Create a governance and scheme
---
The next step would be to create a governance with the 3 nodes that we have previously raised, as well as the definition of the schema and the policies. To do this, we run the script **create_governance.sh**, which will request the port of the node from which we want to create the governance and then we will request the **Controller ID** of each node. As this is a tutorial we have defined the following as members:

```json
    "members": [
        {
            "id": "Company-0",
            "tags": {},
            "description": "Headquarters in 0",
            "key": <ControllerID of node-0>
        },
        {
            "id": "Company-1",
            "tags": {},
            "description": "Headquarters in 1",
            "key": <ControllerID of node-1>
        },
        {
            "id": "Company-2",
            "tags": {},
            "description": "Headquarters in 2",
            "key": <ControllerID of node-2>
        }
    ]
```

And we have specified the following scheme:

```json
    "schemas": [
        {
            "id": "Test",
            "tags": {},
            "content": {
                "type": "object",
                "additionalProperties": false,
                "required": [
                    "temperature",
                    "location",
                    "batch"
                ],
                "properties": {
                    "temperature": {"type": "integer"},
                    "location": {"type": "string" },
                    "batch": {
                        "type": "object",
                        "additionalProperties": false,
                        "required": [ "weight", "origin" ],
                        "properties": {
                            "weight": {"type": "number", "minimum": 0},
                            "origin": {"type": "string"}
                        }
                    }
                }
            }
        }
    ]
```


With the following policy:

```json
    "policies": [
        {
            "id": "governance",
            "validation": {
                "quorum": 0.5,
                "validators": [
                    <ControllerID of node-0>,
                    <ControllerID of node-1>,
                    <ControllerID of node-2>
                ]
            },
            "approval": {
                "quorum": 0.5,
                "approvers": [
                    <ControllerID of node-0>,
                    <ControllerID of node-1>,
                    <ControllerID of node-2>
                ]
            },
            "invokation": {
                "owner": { 
                    "allowance": true,
                    "approvalRequired": false
                },
                "set": {
                    "allowance": false,
                    "approvalRequired": false,
                    "invokers": []
                },
                "all": {
                    "allowance": false,
                    "approvalRequired": false
                },
                "external": {
                    "allowance": false,
                    "approvalRequired": false
                }
            }
        },
        {
            "id": "Test",
            "validation": {
                "quorum": 0.5,
                "validators": [
                    <ControllerID of node-0>,
                    <ControllerID of node-1>,
                    <ControllerID of node-2>
                ]
            },
            "approval": {
                "quorum": 0.5,
                "approvers": [
                    <ControllerID of node-0>,
                    <ControllerID of node-1>,
                    <ControllerID of node-2>
                ]
            },
            "invokation": {
                "owner": {
                    "allowance": true,
                    "approvalRequired": false
                },
                "set": {
                    "allowance": false,
                    "approvalRequired": false,
                    "invokers": []
                },
                "all": {
                    "allowance": false,
                    "approvalRequired": false
                },
                "external": {
                    "allowance": false,
                    "approvalRequired": false
                }
            }
        }
    ]
```

This governance is created by using **create_governance.sh**. When we run it, script will be asking us for the controller ids of the members. We introduce it and, if everything went correctly, you will have an output **similar** to the following:
```bash
./create_governance.sh 
Requesting controllerID.
Enter the controllerID of the node that will be inserted (empty to exit): EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
Enter the controllerID of the node that will be inserted (empty to exit): EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
Enter the controllerID of the node that will be inserted (empty to exit): EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
Enter the controllerID of the node that will be inserted (empty to exit): 
<STOP>


Requesting HTTP PORT. 
Enter the API Rest Port where governance will be created: 10001
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  8693  100  2483  100  6210   116k   290k --:--:-- --:--:-- --:--:--  424k
{"request":{"Create":{"governance_id":"","schema_id":"governance","namespace":"","payload":{"Json":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"}}},"request_id":"JTwVwm3Fs64AETzhfKjOLaQDHjg0OPlcUjIvW9JG4wr0","timestamp":1673878863719,"subject_id":"Jkc2uOqWtjFsVf_oRRqHrPvf1Zx07pOXRh0LSj42jbS4","sn":0}
```

Take note of the **ID** of our governance appears at the end, in the example:
```json
 ... "subject_id":"Jkc2uOqWtjFsVf_oRRqHrPvf1Zx07pOXRh0LSj42jbS4" ...
```
It's important for the next step.

### Create a subject
---
To create a subject, we will use the **create_subject.sh** script. When running this script, we will be prompted for the following information:
1. The **port** from the node we want to create the subject.
2. The **ID** of the governance in which we want to create the subject. It is specificed at the output's end of the previous script executed.

    ```bash
        Governance successfully created. Governance ID: "J4cS48fg6mhKg2oOfbJSf9qp8WcvFh1gCLkWmhI7PYTU"
    ```
3. The **namespace** you want for the subject. It could be any name.
4. The **temperature** at which the product is stored.
5. The **location** where the product is located.
6. The **weight** of the product.
7. The **origin** of the product.

If everything went correctly, you will have an output similar to the following:

```bash
    Requesting port: 
    Enter the port of the node on which you want to create the subject: 3001

    Requesting governance: 
    Enter the ID of the governance in which you want to create the subject (example: J1ZZ57u4PpvTl3apJ0BQrRFrQ1ftMC4XXg-kd9CkZC3E): Jkc2uOqWtjFsVf_oRRqHrPvf1Zx07pOXRh0LSj42jbS4

    Requesting namespace: 
    Enter the namespace of the subject (example: Namespace1): namespace_tutorial

    Requesting product temperature: 
    Enter the temperature at which the product is stored (example: 30): 20

    Requesting product location: 
    Enter the location where the product is located (example: Spain): Portugal

    Requesting product weight: 
    Enter product weight (example: 33): 30

    Requesting product origin: 
    Enter the product origin (example: Spain): France

    % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                    Dload  Upload   Total   Spent    Left  Speed
    100  1670  100  1262  100   408   127k  42148 --:--:-- --:--:-- --:--:--  181k
    Response: {"request":{"Create":{"governance_id":"JQmHG7Ykk4FBuwgSNCHFP6BzuaV0hWf__ukR3m5r5KwE","schema_id":"Test","namespace":"namespace_tutorial","payload":{"Json":"{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}"}}},"request_id":"JyncknOliQiepfAnff2LKvDXng9Eu0kY2KkkhzROX2Gw","timestamp":1673891160906,"subject_id":"JZLfIMPpzfcvEky_Inlzt1EpIGqokvNOp1D6-SrRp78E","sn":0}
```

### Create an event
---
To create an event, we will use the **create_event.sh** script. When running this script, we will be prompted for the following information:
1. The **port** from the node we want to create the event. This node should be the owner of the subject.
2. The **ID** of the subject to which we want to create an event. It's specified at the output's end of the previous executed script:

    ```bash
        Subject ID created: JUrbX__QIIdsZVyoCz1yLKkYGltAYfvF5aKOYsSvZIy4
    ```
3. The **temperature** at which the product is stored.
4. The **location** where the product is located.
5. The **weight** of the product.
6. The **origin** of the product.

If everything went correctly, you will have an output similar to the following:

```bash
    Requesting port: 
    Enter the port of the node from which you want to perform the event: 3001

    Requesting subject: 
    Enter the ID of the subject to which you want to perform an event (example: Jjvs-Kk5FHRVwfktXEiH7y12CYZmV3sSBEyxwzECVA9Y): JUrbX__QIIdsZVyoCz1yLKkYGltAYfvF5aKOYsSvZIy4

    Requesting product temperature: 
    Enter the temperature at which the product is stored (example: 30): 7

    Requesting product location: 
    Enter the location where the product is located (example: Spain): Spain

    Requesting product weight: 
    Enter product weight (example: 33): 30

    Requesting product origin: 
    Enter the product origin (example: Spain): Spain

    % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                    Dload  Upload   Total   Spent    Left  Speed
    100  1531  100  1247  100   284  85917  19567 --:--:-- --:--:-- --:--:--  106k
    Response:
    {"request":{"State":{"subject_id":"JZLfIMPpzfcvEky_Inlzt1EpIGqokvNOp1D6-SrRp78E","payload":{"Json":"{\"batch\":{\"origin\":\"Spain\",\"weight\":33},\"location\":\"Spain\",\"temperature\":30}"}}},"request_id":"JubI7DkNETG32kzNVq3Qsv6kIA4_NTE3NazPiEKJwtLI","timestamp":1673892304209,"subject_id":"JZLfIMPpzfcvEky_Inlzt1EpIGqokvNOp1D6-SrRp78E","sn":1}
```

### Display subjects of a node
---
To visualize the subjects contained in a node, we can make use of the **get_subjects.sh** script, which will ask us for the port of the node we want to see the subjects of.

If everything went correctly, you will have an output similar to the following:

```bash
    Requesting port: 
    Enter the port of the node you want to check subjects on: 3001

    % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                    Dload  Upload   Total   Spent    Left  Speed
    100  1522  100  1522    0     0  1352k      0 --:--:-- --:--:-- --:--:-- 1486k
    Response:
    [{"subject_id":"JQmHG7Ykk4FBuwgSNCHFP6BzuaV0hWf__ukR3m5r5KwE","governance_id":"","sn":0,"public_key":"EUFL-LGFO0OIXgr0INuJqlq5eedMeGU-imU-KUXOwQBE","namespace":"","schema_id":"governance","owner":"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y","properties":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"},{"subject_id":"JZLfIMPpzfcvEky_Inlzt1EpIGqokvNOp1D6-SrRp78E","governance_id":"JQmHG7Ykk4FBuwgSNCHFP6BzuaV0hWf__ukR3m5r5KwE","sn":1,"public_key":"EwRXbmYZCQfus6Aa5jNu9xdXIVHc9z5BQ13PRNs0mu7I","namespace":"SpaceTest","schema_id":"Test","owner":"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E","properties":"{\"batch\":{\"origin\":\"Spain\",\"weight\":33},\"location\":\"Spain\",\"temperature\":30}"}]
```

## 📎 Annotations
- To display the **Controller ID** of the nodes, you must execute the following command:

    ```bash
        docker logs "CONTAINER ID"
    ```

    Where the **CONTAINER ID** of the node can be viewed by executing the following command:
    
    ```bash
        docker ps
    ```

    As a result, the following results were obtained:

    ```bash
    CONTAINER ID   IMAGE                           COMMAND          CREATED          STATUS          PORTS     NAMES
    bd066207424f   opencanarias/taple-client:0.1   "taple-client"   49 minutes ago   Up 49 minutes             node-2
    ba33b0c40bf3   opencanarias/taple-client:0.1   "taple-client"   49 minutes ago   Up 49 minutes             node-1
    e97418ead044   opencanarias/taple-client:0.1   "taple-client"   50 minutes ago   Up 50 minutes             node-0
    ```
    
- Once you have finished the tutorial if you wish, you can make use of the script named **finish.sh** which will stop and delete the containers used for the tutorial.

    If everything went correctly, you will have an output similar to the following:

    ```bash
    $ ./finish.sh 
    a8afb08b0e6f
    4a4b3f776211
    46f2a1d6b8be
    Docker containers has been stopped and delete correctly
    ```
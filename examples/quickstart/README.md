<div align="center">

# Tutorial for setting up a test network using scripts

<br>
    <p align="center">
    <img src= "https://raw.githubusercontent.com/opencanarias/public-resources/master/images/taple-logo-readme.png" width=300px>
    </p>
<br>
</div>

## Índice
- **[🗣 Description](#-description)**
- **[💻 Setting Up Environment](#-setting-up-environment)**
    - **[Running a 3-node network](#running-a-3-node-network)**
      - **[Launching first node](#launching-first-node)**
      - **[Launching second and third node](#launching-second-and-third-node)**
    - **[Create a governance and scheme](#create-a-governance-and-scheme)**
    - **[Create a subject](#create-a-subject)**
    - **[Create an event](#create-an-event)**
    - **[Create more subjects](#create-more-subjects)**
    - **[Display subjects of a node](#display-subjects-of-a-node)**
- **[Stopping Nodes](#stopping-nodes)**

<br />

## 🗣 Description
In this section, you will find a tutorial on how to set up a simple case of use for foods batches traceability using **TAPLE** technology by means of scripts.

## 💻 Setting Up Environment

Before we begin, let's prepare the environment so that the execution of the tutorial is as satisfactory as possible:

1. Revise that you meet the [requirements](https://doc-taple.opencanarias.com/docs/develop/requirements).
2. Let's clone the **Taple Client** repository where the scripts we will use later are stored:
    ```bash
    $ git clone https://github.com/opencanarias/taple-client.git ~/taple-client
    ```
3. We move to the folder where the scripts are located:
    ```
    $ cd ~/taple-client/examples/quickstart/scripts/
    ```

## Running a 3-node network
---

### Launching first node

Before launching any TAPLE node, we need cryptographic material to identify the node, to accomplish this, before each launch,  we need to run **taple-keygen**, getting the material we need:

```bash title="Node 0. Getting cryptographic material"
$ taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e
CONTROLLER ID ED25519: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
PeerID: 12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
```
We will save these values for later. For now, let's raise a TAPLE node by executing the script **launch_node.sh**:

```bash title="Node 0. Input Number ID"
$ ./launch_node.sh
Enter a number to identify the node that will be deployed: 0
```
First, it will prompt for an integer. This number must be unique and will be used to determine the HTTP and P2P port that will be assigned.


```bash title="Node 0. Giving cryptographic material"
Requesting cryptographic material.
Enter secret key: 01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e
```
Next, it will prompt for a **secret key** . This value is the same obtained previously by running **taple-keygen** and will be the one we will enter.

```bash title="Node 0. Setting known nodes"
Requesting known nodes.
Enter address of a known node (leave empty to stop requesting):
<STOP>
```
Now it's the turn for the known nodes that it will use as bootstrap nodes. Since this is our first node, we will not enter anything and will proceed by pressing ENTER. With this, we will have all the necessary parameters to run TAPLE.

```bash title="Node 0. Launching node"
[2023-01-17T16:25:24Z INFO  taple_client] AppSettings { network: NetworkSettings { p2p_port: 11000, addr: "/ip4/0.0.0.0/tcp", known_nodes: [] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("01e1ad914f378fe4b52b2b2bea52aded744e1a3a056fec7a436a5bf0226b9d6e"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data0" }, http_addr: "0.0.0.0", http_port: 10000, x_api_key: None, swagger_ui: false }
[2023-01-17T16:25:24Z INFO  taple_client] Controller ID: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
[2023-01-17T16:25:24Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-17T16:25:24Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-17T16:25:24Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
[2023-01-17T16:25:24Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
```

### Launching second and third node

We will repeat previous steps. Since we are going to raise two nodes, we will run **taple-keygen** and **launch_node.sh** twice.

Lets start with the second node. First we get the cryptographic material:

```bash title="Node 1. Getting cryptographic material for Node 1"
$ taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015
CONTROLLER ID ED25519: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
PeerID: 12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk
```

Later, following the same pattern described [before](./quickstart#launching-first-node), we will launch the second node, with the difference that now the address to the known node is to the first one deployed:

```bash title="Node 0 Address"
...
"/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"
...
```

```bash title="Node 1. Launching Node 1"
$ ./launch_node.sh 
0.1: Pulling from opencanarias/taple-client
Digest: sha256:d3382b5407d6a494f09fabec7d17fb61066751605020d014528fc86548687b8d
Status: Image is up to date for opencanarias/taple-client:0.1
docker.io/opencanarias/taple-client:0.1
Image downloaded successfully.

Enter a number to identify the node that will be deployed: 1

Requesting cryptographic material.
Enter secret key: 0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015
Requesting known nodes.
Enter address of known node (leave empty to stop requesting):/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
Enter address of known node (leave empty to stop requesting):
<STOP>

[2023-01-17T16:26:17Z INFO  taple_client] AppSettings { network: NetworkSettings { p2p_port: 11001, addr: "/ip4/0.0.0.0/tcp", known_nodes: ["/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("0a17286d8dd6a234b5b6140aeed93eb150cf4c24a1dfd14df6325d4c7401f015"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data1" }, http_addr: "0.0.0.0", http_port: 10001, x_api_key: None, swagger_ui: false }
[2023-01-17T16:26:17Z INFO  taple_client] Controller ID: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
[2023-01-17T16:26:17Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-17T16:26:17Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-17T16:26:17Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
[2023-01-17T16:26:17Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11001/p2p/12D3KooWCMc5Lp51YC9chjJ47b6P7puc5pXjKhqKAZpWHtQ9agvk"
```

We repeat the same process for the third node:

```bash title="Node 2. Getting cryptographic material for Node 2"
$ taple-keygen ed25519
keygen
["taple-keygen", "ed25519"]
PRIVATE KEY ED25519 (HEX): 09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b
CONTROLLER ID ED25519: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
PeerID: 12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S
```

```bash title="Node 2. Launching Node 2"
$ ./launch_node.sh 
0.1: Pulling from opencanarias/taple-client
Digest: sha256:d3382b5407d6a494f09fabec7d17fb61066751605020d014528fc86548687b8d
Status: Image is up to date for opencanarias/taple-client:0.1
docker.io/opencanarias/taple-client:0.1
Image downloaded successfully.

Enter a number to identify the node that will be deployed: 2

Requesting cryptographic material.
Enter secret key: 09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b
Requesting known nodes.
Enter address of known node (leave empty to stop requesting):/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX
Enter address of known node (leave empty to stop requesting):
<STOP>

[2023-01-17T16:26:45Z INFO  taple_client] AppSettings { network: NetworkSettings { p2p_port: 11002, addr: "/ip4/0.0.0.0/tcp", known_nodes: ["/ip4/127.0.0.1/tcp/11000/p2p/12D3KooWC5kULse9cPS9vhKfmntwjcLnde6RqHjbqYn4wFj8yxfX"] }, node: NodeSettings { key_derivator: Ed25519, secret_key: Some("09319f4332335e96e3335b6970c5834fe946d87211df47eda63eef8494e6513b"), seed: None, digest_derivator: Blake3_256, replication_factor: 0.25, timeout: 3000, passvotation: 0, dev_mode: false }, database: DatabaseSettings { path: "/tmp/data2" }, http_addr: "0.0.0.0", http_port: 10002, x_api_key: None, swagger_ui: false }
[2023-01-17T16:26:45Z INFO  taple_client] Controller ID: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
[2023-01-17T16:26:45Z INFO  network::network] RED: "/ip4/172.17.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-17T16:26:45Z INFO  network::network] RED: "/ip4/25.33.212.160/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-17T16:26:45Z INFO  network::network] RED: "/ip4/192.168.1.46/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
[2023-01-17T16:26:45Z INFO  network::network] RED: "/ip4/127.0.0.1/tcp/11002/p2p/12D3KooWKvk51kTNB9ARdACDpsohCX3aaBA4Z2tND4FXQwuSb95S"
```

We can check everything is working fine by checking that our three terminals are up and running our three TAPLE nodes.

## Create a governance and scheme
---
The next step would be to create a governance. Since we are working with three nodes, we will define three members inside the governance. Each one being a validator of the subject , because of that, in policies, we are assigning them as validators with a [quorum](../technology/governance) of 50% so we only need the signature of two of the three nodes to accept it inside the ledger. Also, to model the use case, we are defining an schema so our subject will store three properties: `temperature`, `location` and `batch` . 

In the next code block we can see our governance :
<details>
  <summary>Complete Governance JSON</summary>

```json 
{
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
    ],
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
    ],
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
                "quorum": 0,
                "approvers": []
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
}
```
</details>

This governance is created by using **create_governance.sh**. When we run it, script will be asking us for:
1. **Controller ids** of the members.For the example they were:
    ```bash title="Controller id for Node 0"
    ...
    [2023-01-17T16:25:24Z INFO  taple_client] Controller ID: EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
    ....
    ```
    ```bash title="Controller id for Node 1"
    ...
    [2023-01-17T16:26:17Z INFO  taple_client] Controller ID: EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
    ...
    ```
    ```bash title="Controller id for Node 2"
    ...
    [2023-01-17T16:26:45Z INFO  taple_client] Controller ID: EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
    ...
    ```
2. **Number ID** from the node we want to create the governance. For the example it will be number `0`. 
   
We introduce it you will have an output **similar** to the following:
```bash title="Executing create_governance.sh for Node 0"
./create_governance.sh 
Requesting controllerID.
Enter the controllerID of the node that will be inserted (empty to exit): EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y
Enter the controllerID of the node that will be inserted (empty to exit): EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E
Enter the controllerID of the node that will be inserted (empty to exit): EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E
Enter the controllerID of the node that will be inserted (empty to exit): 
<STOP>

Requesting HTTP PORT.
Enter the number ID of the NODE where request for governance creation will be sent: 0

{"request":{"Create":{"governance_id":"","schema_id":"governance","namespace":"","payload":{"Json":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"}}},"request_id":"JQr8oW0CqCoUNWtU-CYBcK5Wn6s_T3We0oleOzls1NzU","timestamp":1673972899640,"subject_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":0}
```

If everything worked correctly we should see a similar output in the terminals of our nodes:

```bash title="Node-0 Output. Governance Owner"
...
[2023-01-17T16:28:19Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y event 0 signed
[2023-01-17T16:28:19Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y created
[2023-01-17T16:28:19Z INFO  rest::handlers] data: Ok(RequestData { request: Create(CreateRequest { governance_id: DigestIdentifier { derivator: Blake3_256, digest: [] }, schema_id: "governance", namespace: "", payload: Json("{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}") }), request_id: "JQr8oW0CqCoUNWtU-CYBcK5Wn6s_T3We0oleOzls1NzU", timestamp: 1673972899640, subject_id: Some("JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y"), sn: Some(0) })
```

```bash title="Node-1 Output."
...
[2023-01-17T16:28:19Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y event 0 signed
[2023-01-17T16:28:19Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y event 0 signed
```
```bash title="Node-2 Output."
...
[2023-01-17T16:28:22Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y event 0 signed
[2023-01-17T16:28:22Z INFO  protocol::command_head_manager::inner_manager] Subject JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y event 0 signed
```

## Create a subject
---
To create a subject, we will use the **create_subject.sh** script. When running this script, we will be prompted for the following information:
1. The **number ID** from the node we want to create the subject. For the example it will be number `2`.
2. The **ID** of the governance in which we want to create the subject. It is specified at the output's end of the previous script executed. For the example it was:
    ```json
    ... "subject_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y" ...
    ```

    :::caution
    The subject_id you will get will differ from the one displayed in the example.
    :::
3. The **temperature** at which the product is stored.
4. The **location** where the product is located.
5. The **weight** of the product.
6. The **origin** of the product.

```bash title="Executing create_subject.sh for Node 2"
    Requesting port.
    Enter the number ID of the node on which you want to create the subject: 2

    Requesting governance.
    Enter the ID of the governance in which you want to create the subject (example: J1ZZ57u4PpvTl3apJ0BQrRFrQ1ftMC4XXg-kd9CkZC3E): JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y

    Requesting product temperature. 
    Enter the temperature at which the product is stored (example: 30): 20

    Requesting product location. 
    Enter the location where the product is located (example: Spain): Portugal

    Requesting product weight.
    Enter product weight (example: 33): 30

    Requesting product origin. 
    Enter the product origin (example: Spain): France

    Response: {"request":{"Create":{"governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","schema_id":"Test","namespace":"","payload":{"Json":"{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}"}}},"request_id":"JWT_FVf79PSoeorsV4Hqekn44TXMtME_F2azE_82aGH8","timestamp":1673973038997,"subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ","sn":0}
```

If everything went correctly, you will have an output similar to the following in your terminals running the nodes:

```bash title="Node-0 Output"
...
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 0 signed
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 0 signed
```

```bash title="Node-1 Output"
...
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 0 signed
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 0 signed
```

```bash title="Node-2 Output. Subject Owner"
...
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 0 signed
[2023-01-17T16:30:39Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ created
[2023-01-17T16:30:39Z INFO  rest::handlers] data: Ok(RequestData { request: Create(CreateRequest { governance_id: DigestIdentifier { derivator: Blake3_256, digest: [173, 128, 43, 83, 78, 109, 192, 178, 199, 178, 217, 40, 211, 32, 43, 238, 64, 1, 133, 206, 58, 33, 17, 153, 156, 150, 21, 202, 84, 83, 103, 198] }, schema_id: "Test", namespace: "", payload: Json("{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}") }), request_id: "JWT_FVf79PSoeorsV4Hqekn44TXMtME_F2azE_82aGH8", timestamp: 1673973038997, subject_id: Some("JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ"), sn: Some(0) })
```
## Create an event
---
To create an event, we will use the **create_event.sh** script. When running this script, we will be prompted for the following information:
1. The **number ID** from the node we want to create the subject. For the example it will be number `2`.
2. The **ID** of the subject to which we want to create an event. It's specified at the output's end of the previous executed script.For the example it was:
    ```json
        ... "subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ"...
    ```
    :::caution
    The subject_id you will get will differ from the one displayed in the example.
    :::
3. The **temperature** at which the product is stored.
4. The **location** where the product is located.
5. The **weight** of the product.
6. The **origin** of the product.

```bash title="Creating event from Node 2"
    Requesting port.
    Enter the number ID of the node from which you want to perform the event: 2

    Requesting subject. 
    Enter the ID of the subject to which you want to perform an event (example: Jjvs-Kk5FHRVwfktXEiH7y12CYZmV3sSBEyxwzECVA9Y): JPnjBHk35OmYM-iljWlrv0Xd7a6UzdavM6lZlwG6LkEI

    Requesting product temperature. 
    Enter the temperature at which the product is stored (example: 30): 7

    Requesting product location: 
    Enter the location where the product is located (example: Spain): Spain

    Requesting product weight. 
    Enter product weight (example: 33): 30

    Requesting product origin. 
    Enter the product origin (example: Spain): Spain

    Response:
    {"request":{"State":{"subject_id":"JZLfIMPpzfcvEky_Inlzt1EpIGqokvNOp1D6-SrRp78E","payload":{"Json":"{\"batch\":{\"origin\":\"Spain\",\"weight\":33},\"location\":\"Spain\",\"temperature\":30}"}}},"request_id":"JubI7DkNETG32kzNVq3Qsv6kIA4_NTE3NazPiEKJwtLI","timestamp":1673892304209,"subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ","sn":1}
```

If everything went correctly, you will have an output similar to the following in your terminals running the nodes:

```bash title="Node-0 Output"
...
[2023-01-17T16:33:13Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 1 signed
```

```bash title="Node-1 Output"
...
[2023-01-17T16:33:13Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 1 signed
```

```bash title="Node-2 Output. Subject Owner"
...
[2023-01-17T16:33:13Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 1 signed
[2023-01-17T16:33:13Z INFO  protocol::command_head_manager::inner_manager] Subject JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ event 1 created
[2023-01-17T16:33:13Z INFO  rest::handlers] data: Ok(RequestData { request: State(StateRequest { subject_id: DigestIdentifier { derivator: Blake3_256, digest: [62, 120, 193, 30, 77, 249, 58, 102, 12, 250, 41, 99, 90, 90, 239, 209, 119, 123, 107, 165, 51, 117, 171, 204, 234, 86, 101, 192, 110, 139, 144, 66] }, payload: Json("{\"batch\":{\"origin\":\"Spain\",\"weight\":30},\"location\":\"Spain\",\"temperature\":7}") }), request_id: "JSbYT3cgRgT8Svh8V3sDmxn01QAlFeDmg00A9bmMv6vY", timestamp: 1673969593828, subject_id: Some("JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ"), sn: Some(1) })
```
## Create more subjects

Until now we have created a subject that model a food batch . TAPLE allows us to create more subjects dynamically. Lets create more subjects by repeating steps from ["Create a subject"](./quickstart#create-a-subject):

1. Create subject:
    ```bash title="Creating subject from Node 1"
    $ ./create_subject.sh 
    Requesting port.
    Enter the number ID of the node on which you want to create the subject: 1

    Requesting governance.
    Enter the ID of the governance in which you want to create the subject (example: J1ZZ57u4PpvTl3apJ0BQrRFrQ1ftMC4XXg-kd9CkZC3E): JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y

    Requesting product temperature.
    Enter the temperature at which the product is stored (example: 30): 30

    Requesting product location.
    Enter the location where the product is located (example: Spain): Spain

    Requesting product weight.
    Enter product weight (example: 33): 33

    Requesting product origin.
    Enter the product origin (example: Spain): Spain

    Response: {"request":{"Create":{"governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","schema_id":"Test","namespace":"","payload":{"Json":"{\"batch\":{\"origin\":\"Spain\",\"weight\":33},\"location\":\"Spain\",\"temperature\":30}"}}},"request_id":"JSohR7uWstwQd5j4hx8hDccb3FKN_hnmCoGHrm79h_wk","timestamp":1673973188427,"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","sn":0}
    ```
2. Update subject state:
    ```bash title="Updating subject from Node 1"
    $ ./create_event.sh 
    Requesting port.
    Enter the number ID of the node from which you want to perform the event: 1

    Requesting subject.
    Enter the ID of the subject to which you want to perform an event (example: Jjvs-Kk5FHRVwfktXEiH7y12CYZmV3sSBEyxwzECVA9Y): J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4

    Requesting product temperature.
    Enter the temperature at which the product is stored (example: 30): 0

    Requesting product location.
    Enter the location where the product is located (example: Spain): England

    Requesting product weight.
    Enter product weight (example: 33): 0

    Requesting product origin.
    Enter the product origin (example: Spain): Spain

    Response:
    {"request":{"State":{"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","payload":{"Json":"{\"batch\":{\"origin\":\"Spain\",\"weight\":0},\"location\":\"England\",\"temperature\":0}"}}},"request_id":"JRllKPd-4ONa7iDTEVUvLlNVMsRpAQjIRQTasq7RbdEI","timestamp":1673973254635,"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","sn":1}
    ```
3. Check node terminals output:
    ```bash title="Node-0 Output"
    ...
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 0 signed
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 0 signed
    [2023-01-17T16:36:35Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 1 signed
    ```

    ```bash title="Node-1 Output. Subject Owner"
    ...
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 0 signed
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 created
    [2023-01-17T16:35:39Z INFO  rest::handlers] data: Ok(RequestData { request: Create(CreateRequest { governance_id: DigestIdentifier { derivator: Blake3_256, digest: [173, 128, 43, 83, 78, 109, 192, 178, 199, 178, 217, 40, 211, 32, 43, 238, 64, 1, 133, 206, 58, 33, 17, 153, 156, 150, 21, 202, 84, 83, 103, 198] }, schema_id: "Test", namespace: "", payload: Json("{\"batch\":{\"origin\":\"Spain\",\"weight\":33},\"location\":\"Spain\",\"temperature\":30}") }), request_id: "JSohR7uWstwQd5j4hx8hDccb3FKN_hnmCoGHrm79h_wk", timestamp: 1673973188427, subject_id: Some("J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4"), sn: Some(0) })
    [2023-01-17T16:36:35Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 1 signed
    [2023-01-17T16:36:35Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 1 created
    [2023-01-17T16:36:35Z INFO  rest::handlers] data: Ok(RequestData { request: State(StateRequest { subject_id: DigestIdentifier { derivator: Blake3_256, digest: [62, 120, 193, 30, 77, 249, 58, 102, 12, 250, 41, 99, 90, 90, 239, 209, 119, 123, 107, 165, 51, 117, 171, 204, 234, 86, 101, 192, 110, 139, 144, 66] }, payload: Json("{\"batch\":{\"origin\":\"Spain\",\"weight\":30},\"location\":\"Spain\",\"temperature\":7}") }), request_id: "JRllKPd-4ONa7iDTEVUvLlNVMsRpAQjIRQTasq7RbdEI", timestamp: 1673973254635, subject_id: Some("J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4"), sn: Some(1) })
    ```
    ```bash title="Node-2 Output."
    ...
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 0 signed
    [2023-01-17T16:35:39Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 0 signed
    [2023-01-17T16:36:35Z INFO  protocol::command_head_manager::inner_manager] Subject J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4 event 1 signed
    ```
4. Display subjects:

## Display subjects of a node
---
This step is useful to validate that the network is working, and everybody has the same copy of the microledger. To visualize the subjects contained in a node, we can make use of the **get_subjects.sh** script, which will ask us for the port of the node we want to see the subjects of.

If everything went correctly, you will have an output similar to the following:

```bash title="get_subjects.sh . Node 0"
    Requesting port. 
    Enter the number ID of the node you want to check subjects on: 0

    Response:
    [{"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":1,"public_key":"EeYrJkISQP2RallGct7HXhqAxSo4LRnMtnW0vUWodoPA","namespace":"","schema_id":"Test","owner":"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E","properties":"{\"batch\":{\"origin\":\"Spain\",\"weight\":0},\"location\":\"England\",\"temperature\":0}"},{"subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":0,"public_key":"EZxOq58lef5BIoC-ji4KZjkzjqxLJe6z2DGyAGLNmqqw","namespace":"","schema_id":"Test","owner":"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E","properties":"{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}"},{"subject_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","governance_id":"","sn":0,"public_key":"EV7y-hRryeYuzW1FCjuwSyuUe97ojiWzDiAn-xM1Elmc","namespace":"","schema_id":"governance","owner":"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y","properties":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"}]
```

```bash title="get_subjects.sh . Node 1"
    Requesting port. 
    Enter the number ID of the node you want to check subjects on: 1

    Response:
    [{"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":1,"public_key":"EeYrJkISQP2RallGct7HXhqAxSo4LRnMtnW0vUWodoPA","namespace":"","schema_id":"Test","owner":"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E","properties":"{\"batch\":{\"origin\":\"Spain\",\"weight\":0},\"location\":\"England\",\"temperature\":0}"},{"subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":0,"public_key":"EZxOq58lef5BIoC-ji4KZjkzjqxLJe6z2DGyAGLNmqqw","namespace":"","schema_id":"Test","owner":"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E","properties":"{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}"},{"subject_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","governance_id":"","sn":0,"public_key":"EV7y-hRryeYuzW1FCjuwSyuUe97ojiWzDiAn-xM1Elmc","namespace":"","schema_id":"governance","owner":"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y","properties":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"}]
```

```bash title="get_subjects.sh . Node 2"
    Requesting port. 
    Enter the number ID of the node you want to check subjects on: 2

    Response:
    [{"subject_id":"J1_Uw6dWAMmOpK6uJxJ0gmXpr8x-v6Of6tFDxQUX5Eu4","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":1,"public_key":"EeYrJkISQP2RallGct7HXhqAxSo4LRnMtnW0vUWodoPA","namespace":"","schema_id":"Test","owner":"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E","properties":"{\"batch\":{\"origin\":\"Spain\",\"weight\":0},\"location\":\"England\",\"temperature\":0}"},{"subject_id":"JOG5n5ajSAizYBD78py6CqzgYSBz49w6w569SztPnNkQ","governance_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","sn":0,"public_key":"EZxOq58lef5BIoC-ji4KZjkzjqxLJe6z2DGyAGLNmqqw","namespace":"","schema_id":"Test","owner":"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E","properties":"{\"batch\":{\"origin\":\"France\",\"weight\":30},\"location\":\"Portugal\",\"temperature\":20}"},{"subject_id":"JrYArU05twLLHstko0yAr7kABhc46IRGZnJYVylRTZ8Y","governance_id":"","sn":0,"public_key":"EV7y-hRryeYuzW1FCjuwSyuUe97ojiWzDiAn-xM1Elmc","namespace":"","schema_id":"governance","owner":"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y","properties":"{\"members\":[{\"description\":\"Headquarters in 1\",\"id\":\"Company-1\",\"key\":\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"tags\":{}},{\"description\":\"Headquarters in 2\",\"id\":\"Company-2\",\"key\":\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"tags\":{}},{\"description\":\"Headquarters in 3\",\"id\":\"Company-3\",\"key\":\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\",\"tags\":{}}],\"policies\":[{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"governance\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}},{\"approval\":{\"approvers\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"],\"quorum\":0.5},\"id\":\"Test\",\"invokation\":{\"all\":{\"allowance\":false,\"approvalRequired\":false},\"external\":{\"allowance\":false,\"approvalRequired\":false},\"owner\":{\"allowance\":true,\"approvalRequired\":false},\"set\":{\"allowance\":false,\"approvalRequired\":false,\"invokers\":[]}},\"validation\":{\"quorum\":0.5,\"validators\":[\"EIahN95FYIOO5BwbEuGQ6VPFZTPTTjkrCTu3VFGM7O2Y\",\"EJbgafSSzx3QZwem79ypwxp8P4Q2kcrMZtdpzgPF1d3E\",\"EljceLTouD478TdoKVhOsSkfX6DBQ7G6M8yD0ChXhl4E\"]}}],\"schemas\":[{\"content\":{\"additionalProperties\":false,\"properties\":{\"batch\":{\"additionalProperties\":false,\"properties\":{\"origin\":{\"type\":\"string\"},\"weight\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"weight\",\"origin\"],\"type\":\"object\"},\"location\":{\"type\":\"string\"},\"temperature\":{\"type\":\"integer\"}},\"required\":[\"temperature\",\"location\",\"batch\"],\"type\":\"object\"},\"id\":\"Test\",\"tags\":{}}]}"}]
```

## Stopping Nodes

Once you have finished the tutorial, if you wish, you can make use of the script named **stop_nodes.sh** which will stop and delete the containers used for the tutorial.

```bash
$ ./stop_nodes.sh 
a8afb08b0e6f
4a4b3f776211
46f2a1d6b8be
Containers stopped and disposed of correctly
```
# Architecture

## Components/Units

```mermaid
C4Context

System_Boundary(bProject, "LUX") {
    System_Boundary(bClient, "Client") {
        System_Boundary(bSync, "Sync") {
            System_Boundary(bStaticSync, "Static") {
                System(AssetSync, "Asset")
                System(ScriptingSync, "Scripting")
            }
            System_Boundary(bSmartSync, "Smart") {
                System(EntitySync, "Entity")
                System(ComponentSync, "Component")
            }
        }
        System_Boundary(bComponents, "Components") {
            System(ComponentsWhat, "?")
        }
        System_Boundary(bScripting, "Scripting") {
            System(Scripting, "Scripting")
        }
        System_Boundary(bAssets, "Assets") {
            System(AssetMesh, "Mesh")
            System(AssetRig, "Rig")
            System(AssetAnimation, "Animation")
            System(AssetImage, "Image")
            System(AssetSound, "Sound")
            System(AssetVideo, "Video")
        }
        System_Boundary(bSupports, "Supports") {
            System(SupportVR, "VR")
            System(SupportDesktop, "Desktop")
        }
    }
}

System_Boundary(Backend, "Backend") {
    System(Backend, "Backend")
}
```

## Interactions

### Booststrap

- Main screen -> world selection
- How differnt is interacton in VR?

### Hosting

### Connecting to a host

### Backend (optional)

Backends are not needed for hosting nor for local use. If not logged in the user will remain anonymous and only have access to local disk for storage.

#### Federated login option

Maybe using https://github.com/NexusSocial/NexusProtocol/

### ...


# rconrs
The is a very simple command-line client for the Source RCON (Remote Console) protocol that is described at https://developer.valvesoftware.com/wiki/Source_RCON_Protocol.

## Arguments
| Short    | Long             | Takes Value? | Action |
| -------- | ---------------- | ------------ | ------ |
| ```-h``` | ```--help```     | ✗           | Displays information about the program and how to use it. |
| ```-H``` | ```--host```     | ✓           | The host to connect to. Leave blank for ```localhost```. |
| ```-p``` | ```--port```     | ✓           | The port to connect to. Leave blank for ```25575```. |
| ```-P``` | ```--password``` | ✓           | Required. The password of the RCON server. |

## Use
Upon sucessfully connecting, you will be able to type in your commands and execute them by pressing enter. The command ```quit``` will exit the RCON session and close the program.

This project has been created for use with Minecraft servers, but it probably works for other servers the utilize RCON.

## Looking forward
I hope to add features to this over time, but I'm a working college student so I can't guarantee anything. If you think of something that would be useful or find a bug that should be fixed, feel free to add an issue or make the change yourself and submit a pull request. Please note that this project is under the MIT license so if you contribute something and make a pull request, your changes will be then be under the MIT license. Only contribute if you are comfortable with that.

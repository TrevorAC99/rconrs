# rconrs
The is a very simple client for the Source RCON (Remote Console) protocol that is described at https://developer.valvesoftware.com/wiki/Source_RCON_Protocol. This program should be ran in a terminal, so spin up your favorite one and run it.

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
I'll clean up the code and add a license soon (hopefully), but I'm a working college student so I can't guarantee anything. I put this together in a couple hours and I'm no Rust expert, so I make no claims about the quality of the code.

If you are new to Rust and want to understand what my code is doing, go to https://www.rust-lang.org/ to see how to get started.

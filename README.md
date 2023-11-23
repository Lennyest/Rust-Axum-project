# Rust Axum Project 📄
A small prototype for a rust web framework, using some very bare-bones and not secure login logic, CRUD operations, REST APIs and more!

The system currently has a few testing routes, but the main ones being _/api/tickets_ and _api/login_. If you wish to see how this works fully, check the **examples/quick_dev** folder.

## Building and running 🔨
At the moment, this project is not even close to being finished and it just acting as a prototype for future expansion.
Upon cloning, you should do the following:

**⚠️ This is assuming you are running Windows ⚠️**

- "cargo build" // Build dependencies and then run the following commands
- "cargo watch -c -x -w src/ -x run" // Runs the main server to handle the requests
- "cargo watch -x "run --example quick_dev"" // Runs the client to send the requests to the server.

This project uses the 3000 port, change it if it is in use or you'd prefer to use another for whatever reason.
## Credits
_JeremyChone - Idea & reference material_

## Images/Examples 📷
<img align="center" src="https://i.imgur.com/3LRoySj.png">

<img align="center" src="https://i.imgur.com/CjELB1d.png">

# Onboarding_Project
TypeDB Data Model

### Setup
To get this repository, run the following command inside your git enabled terminal
```bash
$ git clone https://github.com/Might1331/Onboarding_Project.git
```

You will also need typedb server. Download the zip from  https://repo.vaticle.com/#browse/browse:artifact-snapshot:vaticle_typedb%2F07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5%2Ftypedb-server-windows-07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5.zip>.

**Note:: Click on the path to start the download.

Extract the files and run the following command within the "...\typedb-server-windows-07b.." directory
```
$ ./typedb server --server.address=localhost:1729
```

Once the server is running, head over to the root of this repo that you just cloned and run the rust program using the command
```
$ cargo run
```

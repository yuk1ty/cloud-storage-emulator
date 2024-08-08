# E2E testing

Before starting e2e tests, you have to launch the cloud storage emulator on docker in advance.

Run the following command to build:

```
$ docker build . -t cloud-storage-emulator:latest
```

Then hit the following one to run the container in the background:

```
$ docker run -d -p 8000:8000 cloud-storage-emulator:latest
```

# Learning Docker



一个简单的Dockerfile

```dockerfile
# syntax=docker/dockerfile:1
   
FROM node:18-alpine
WORKDIR /app
COPY . .
RUN yarn install --production
CMD ["node", "src/index.js"]
EXPOSE 3000
```



```bash
# build te container image
docker build -t getting-started .

# start the comtainer
docker run -dp 127.0.0.1:3000:3000 getting-started
#-d:后台运行
# -p:
# 暴露localhost 3000到127.0.0.1

# list the containers
docker ps

# stop the container
docker stop <the-container-id>

# remove the container 
docker rm <the-container-id>
# 或直接 docker rm -f <the-container-id>

# push the image 到dockerhub的操作
docker push docker/getting-started
docker tag getting-started YOUR-USER-NAME/getting-started
docker push YOUR-USER-NAME/getting-started

# to build an image for the amd64 platform, use the --platform flag.
docker build --platform linux/amd4 -t YOUR-USER-NAME/getting-started .

```



persist the DB

```bash
# start an ubuntu container that will create a file named /data.txt with a random number between 1 and 10000.
docker run -d ubuntu bash -c "shuf -i 1-10000 -n 1 -o /data.txt && tail -f /dev/null"
# the first portion picks a single random number and writes it to /data.txt. the second command is simply watching a file to keep the container running.

```



```bash
# docker exec -> access the container
docker exec <container-id> cat /data.txt
```



## create a volume

by default, the todo app stores its data in a SPLite database at /etc/todos/todo.db.

```bash
docker volume create todo-db
# start the container and add the --mount option
docker run -dp 127.0.0.1:3000:3000 --mount type=volume,src=todo-db,target=/etc/todos getting-started

# use docker volume inspect to see where is docker storing the data
docker volume inspect todo-db

```

## bind mounts 

a bind mount is another type of mount, which lets you share a directory from the host’s filesystem into the container. 

![image-20230617175412208](C:\Users\W\AppData\Roaming\Typora\typora-user-images\image-20230617175412208.png)



```bash
# start bash in ubuntu container with a bind mount
docker run -it --mount type=bind,src="$(pwd)",target=/src ubuntu bash

# the --mount option tells Docker to create a bind mount, where src is the current working directory on your host machine(getting-started/app),and target is where that directory should appear inside the container(/src).

// stop the interactive container session with Ctrl+D
```

run your app ina a development container

- mount your source code into the container
- intall all dependencies
- start nodemon to watch for filesystem changes



```bash
docker run -dp 127.0.0.1:3000:3000 -w /app --mount type=bind,src="$(pwd)", target=/app node:18-alpine sh-c "yarn install && yarn run dev"
```

- `-dp 127.0.0.1:3000:3000`-run in datached(background) mode and create a port mapping.
- `-w /app`-sets the “working directory” or the current directory that the command will run from.
-  `--mount type=bind,src="$(pwd)",target=/app`-bind mount the current directory from the host into the `/app` directory in the container.
- `node:18-alpine`-the image to use. Note that this is the base image for your app from the Dockerfile.
- `sh -c "yarn install && yarn run dev"`-the command.If you look in the `package.json`,you’ll see that the `dev`script starts `nodemon`.



```bash
# you can watch the logs using docker logs <container-id>
docker logs -f <container-id>
# when you are done watching the logs, exit out by hitting Ctrl+C.
```

each time you make a change and save a file, the `nodemon` process restarts the app inside the container automatically.When you’re done, stop the container and build your new image using:

```
docker build -t getting-started .
```



## multi container apps

“where will MySQL run?”.In general, each container should do one thing and do it well. If you place the two containers on the same networks, they can talk to each other.



```bash
# create the network 
docker network create todo-app

# start the MYSQL container and attach it to the network.
docker run -d --network todo-app --network-alias mysql -v todo-mysql-data:/var/lib/mysql -e MYSQL_ROOT_PASSWORD=secret -e MYSQL_DATABASE=todos mysql:8.0

# to confirm you have the database up and running, connect to the database and verify that is connects.
docker exec -it <mysql-container-id> mysql -u root -p
```



```bash
# in the MYSQL shell, list the databases and verify you see the todo database.
mysql> SHOW DATABASES;

Exit the MySQLshell to return the shell on your machine.
mysql> exit 
```

you are going to make use of the nicolaka/netchoot container,which ships with a lot of tools that are useful for troubleshooting or debugging networking issues.

```bash
# start a new container using the nicolaka/netshoot image.
docker run -it --network todo-app nicolaka/netchoot

# inside the contianer, you're going to use the dig command, which is a useful DNS tool.You're going to look up the IP address for the hostname mysql.
dig mysql
```

you should get output like the following:

```
; <<>> DiG 9.18.8 <<>> mysql
 ;; global options: +cmd
 ;; Got answer:
 ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 32162
 ;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

 ;; QUESTION SECTION:
 ;mysql.				IN	A

 ;; ANSWER SECTION:
 mysql.			600	IN	A	172.23.0.2

 ;; Query time: 0 msec
 ;; SERVER: 127.0.0.11#53(127.0.0.11)
 ;; WHEN: Tue Oct 01 23:47:24 UTC 2019
 ;; MSG SIZE  rcvd: 44
```

In the “ANWSER SECTION”, you will see an `A` record for mysql that resolves to `172.23.0.2`. while `mysql` isn’t normally a valid hostname, Docker was able to reolve it to the IP address of the container that had that network alias.

What this means is that your app only simply needs to connect to a host named `mysql` and it’ll talk to the database.

> **Note**
>
> While using env vars to set connection settings is generally accepted for development, it’s highly discouraged when running applications in production. Diogo Monica, a former lead of security at Docker, [wrote a fantastic blog post](https://diogomonica.com/2017/03/27/why-you-shouldnt-use-env-variables-for-secret-data/) explaining why.
>
> A more secure mechanism is to use the secret support provided by your container orchestration framework. In most cases, these secrets are mounted as files in the running container. You’ll see many apps (including the MySQL image and the todo app) also support env vars with a `_FILE` suffix to point to a file containing the variable.
>
> As an example, setting the `MYSQL_PASSWORD_FILE` var will cause the app to use the contents of the referenced file as the connection password. Docker doesn’t do anything to support these env vars. Your app will need to know to look for the variable and get the file contents.



```bash
# Specify each of the environment variables above, as well as connect the container to your app network

docker run -dp 127.0.0.1:3000:3000 -w /app -v "$(pwd):/app" --network todo-app -e MYSQL_HOST=mysql -e MYSQL_USER=root -e MYSQL_PASSWORD=secret -e MYSQL_DB=todos node:18-alpine sh -c "yarn install && yarn run dev"


# Connect to the mysql database and prove that the items are being written to the database.
docker exec -it <mysql-container-id> mysql -p todos

# in the mysql shell, run the following:
mysql>select * from todo_items;
```



## Docker Compose

Docker Compose is a tool that was developed to help define and share multi-container applications.With Compose, we can create a YAML file to define the services and with a single command, can spin everything up or tear it all down.



Create the Compose file

In the compose file, we’ll start off by defining the list of services(or containers)we want to run as part of our application.



Define the app service

To remember, this was the command we were using to define our app container.

```bash
docker run -dp 127.0.0.1:3000:3000 \
-w /app -v "$(pwd):/app" \
--network todo-app \
-e MYSQL_HOST=mysql \
-e MYSQL_USER=root \
-e MYSQL_PASSWORD=secret \
-e MYSQL_DB=todos \
node:18-alpine \
sh -c "yarn install && yarn run dev"
```



```yaml
services:
  app:
    image: node:18-alpine
    command: sh -c "yarn install && yarn run dev"
    ports:
      - 127.0.0.1:3000:3000
    working_dir: /app
    volumes:
      - ./:/app
    environment:
      MYSQL_HOST: mysql
      MYSQL_USER: root
      MYSQL_PASSWORD: secret
      MYSQL_DB: todos
```

Define the MYSQL service

The command that we used for that container was the following:

```bash
docker run -d \
--network todo-app --network-alias mysql \
-v todo-mysql-data:/var/lib/mysql \
-e MYSQL_ROOT_PASSWORD=secret \
-e MYSQL-DATABASE=todos \
mysql:8.0
```



```
services:
  app:
    image: node:18-alpine
    command: sh -c "yarn install && yarn run dev"
    ports:
      - 127.0.0.1:3000:3000
    working_dir: /app
    volumes:
      - ./:/app
    environment:
      MYSQL_HOST: mysql
      MYSQL_USER: root
      MYSQL_PASSWORD: secret
      MYSQL_DB: todos
  
  mysql:
    image: mysql:8.0
    volumes:
      - todo-mysql-data:/var/lib/mysql
    environment:
      MYSQL_ROOT_PASSWORD: secret
      MYSQL_DATABASE: todos
      
volumes:
  todo-mysql-data:
```

> When we ran the container with `docker run`, the named volume was created automatically.However, that doesn’t happen when running with Compose. We need to define the volume in the top-level `volumes:` section and then specify the mountpoint in the service config.By simply providing only the volume name, the default options are used.



Let’s look at the logs using the `doker compose logs -f` command.The `-f` flag “follows” the log, so will give you live output as it’s generated.



When you’re ready to tear it all down, simply run `docker compose down` .

> Removing Volumes
>
> By default, named volumes in your compose file are NOT removed when running `docker compose down`. If you want to remove the volumes, you will need to add the `--volumes` flag.
>
> The Docker Dashboard does *not* remove volumes when you delete the app stack.

Once tear down, you can switch to another project, run `docker compose up` and be ready to contribute to that project!.



use the `docker image history` command to see the layers.



Layer caching

Let’s look at the Dockerfile we were using one more time.

```dockerfile
# syntax=docker/dockerfile:1
FROM node:18-alpine
WORKDIR /app
COPY . .
RUN yarn install --production
CMD ["node", "src/index.js"]
```

we need to restructure our Dockerfile to help support the caching of the dependencies. For Node-based applications, those dependencies are defined in the `package.json` file.So we copied only that file first, install the dependencies, and then copy in everything else.Then we only recreate the yarn dependencies if there was a change to the package.json.

`.dockerignore` files are an easy way to selectively copy only image relevant files. You can read more abou this [here](https://docs.docker.com/engine/reference/builder/#dockerignore-file). In this case, the `node_modules` folder should be ommitted in the second `COPY` step because otherwise, it would possibly overwirite files which were created by the command in the `RUN` step. For further details on why this is recommended for Node.js applications and other best practices, have a look at their guide on [Dockerizing a Node.js web app](https://nodejs.org/en/docs/guides/nodejs-docker-webapp/).



Maven/Tomcat example

```dockerfile
# syntax=docker/dockerfile:1
FROM maven AS build
COPY . .
RUN mvn package

FROM tomcat
COPY --from=build /app/target/file.war /usr/local/tomcat/webapps
```

In this example, we usee one stage(called`build`)to perform the actual Java build using Maven.In the second stage(starting at `FROM tomcat`),we copy in files from the `build` stage.The final image is only the last stage being created(which can be overriden using the `–-target` flag).

React example

```dockerfile
# syntax=docker/dockerfile:1
FROM node:18 AS build
WORKDIR /app
COPY package* yarn.lock ./
RUN yarn install
COPY public ./public
COPY src ./src
RUN yarn run build

FROM nginx:alpine
COPY --from=build /app/build /usr/share/nginx/html
```

Here, we are using a `node:18` image to perform the build(maxmizing layer caching) and then copying the output into an nginx container.
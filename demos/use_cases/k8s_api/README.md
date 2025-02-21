This demo shows how you can use a publicly hosted rest api and interact it using arch gateway.

# How to run this demo.

Before staring make sure you have completed the pre-requisites [here](https://github.com/katanemo/archgw?tab=readme-ov-file#prerequisites)

In separate window start 1) model server, 2) arch gateway 3) docker container for UI and for debugging 4) tail access logs

1. start model server
   ```
   archgw up --service model_server --foreground
   ```

1. start arch gateway
   ```
   archgw up --service archgw --foreground
   ```

1. start docker container for ui
   ```
   docker compose up
   ```
1. tail access logs
   ```
   tail -F ~/archgw_logs/access_*
   ```

Here is a sample screenshot of the demo in action,

![screenshot](image2.png)
- show usage of get namespace details and get pods
![Demo Screenshot](image.png)

# Price alert system

## How it works?
<img width="720" alt="price-alert-logic" src="https://github.com/harsh-vardhhan/price-alert/assets/3825401/4abff4ab-91c6-4c69-af96-a257fd05116d">

## How to use?
![demo](https://github.com/harsh-vardhhan/price-alert/assets/3825401/e5a6a290-885d-4a50-8758-7fe1846e4c31)

## What API is used for market API?
[Upstox API](https://upstox.com/uplink/)

## How to setup?
### create a .env file in project<br/>

```
CODE=ENTER_CODE_HERE // Restart terminal after entering code
CLIENT_ID=ENTER_CLIENT_ID_HERE  
API_SECRET=ENTER_API_SECRET_HERE  
REDIRECT_URL=ENTER_REDIRECT_URL_HERE  
ACCESS_TOKEN=ENTER_ACCESS_TOKEN_HERE // Restart terminal after entering access token
```

get **CODE** and **ACCESS_TOKEN** after you run the app

<br/><img width="280" alt="app" src="https://github.com/harsh-vardhhan/price-alert/assets/3825401/bccfcacf-8ce3-49ff-acde-463c5993d0e2">

get **API_SECRET**, **REDIRECT_URL** and **CLIENT_ID** from [App section](https://account.upstox.com/developer/apps) Upstox Pro

<img width="512" alt="upstox_app" src="https://github.com/harsh-vardhhan/price-alert/assets/3825401/dd563274-23c4-41aa-8a61-eed67a49f1ee">

### Run the app
```
cargo run
```

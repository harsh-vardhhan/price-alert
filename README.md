# Price alert system
<img width="826" alt="price-alert-logic" src="https://github.com/harsh-vardhhan/price-alert/assets/3825401/4abff4ab-91c6-4c69-af96-a257fd05116d">


**Tracking instrument**:
<br/>
- NIFTY  
- BANKNIFTY

**API:** [Upstox API](https://upstox.com/uplink/trader-api/)

**Why is it written in Rust?**
<br/>The system aims to run all the market hours on the user's desktop. Hence, it aims to run at the lowest memory usage.

![market-price](https://github.com/harsh-vardhhan/price-alert/assets/3825401/5ad92de6-3d4f-4ab6-928e-aa7147065a4e)

## Getting started
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

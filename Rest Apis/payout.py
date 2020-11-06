#!/usr/bin/python3
## Author : Burgerking12

burgerSign = """

   ___                           _    _             _ ____  
  / __\_   _ _ __ __ _  ___ _ __| | _(_)_ __   __ _/ |___ \ 
 /__\// | | | '__/ _` |/ _ \ '__| |/ / | '_ \ / _` | | __) |
/ \/  \ |_| | | | (_| |  __/ |  |   <| | | | | (_| | |/ __/ 
\_____/\__,_|_|  \__, |\___|_|  |_|\_\_|_| |_|\__, |_|_____|
                 |___/                        |___/         
            \u23CF Made with 💘 by Burgerking12
            \u23CF HELLO WORLD VERSION 
            \u23CF JUST TO READ THE PENDING PAYOUTS
            \u23CF ENV=local | --chain=kusama-local 
            \u23CF OR INTERACTED WITH 
            \u23CF ENV=<custom endpoint> | --chain=kusama 

"""

import requests
u256 = 10
LOG = print
Baseurl = "http://127.0.0.1:8080"
PostOption = ["post", "get"]


def checkOptionNetwork():
    data = checkOptionSidecar('/node/version', 'get')
    try:
        return data["chain"]
    except:
        return "NONE"

def checkOpt2_Url():
    LOG("\u26D4 Make Sure run it with ENV=local ↔ " , end='')
    LOG("chain=kusama-local \u26D4 " , end='')
    if checkOptionUrl(Baseurl):
        LOG("")
    else:
        exit()
def checkOptionUrl(url):
    try:
        r = requests.get(url)
        return r.status_code == 200
    except:
        return False

def checkOptionSidecar(path, typeOption, params=None):
    try:
        url = Baseurl+path
        if typeOption not in PostOption:
            raise Exception("Method not allowed")
        client = getattr(requests, typeOption, None)
        response = client(url, data=params, params=params)
        return response.json()
    except:
        raise Exception(" \u26D4 Request Forbidden \u26D4 ")

def checkOptionChain():
    LOG("\u25FB BLOCKCHAIN NETWORK \u25B6 \u25B6 ", end='')
    chain = checkOptionNetwork()
    if chain != "NONE":
        LOG(f" \u23CF \u23CF {chain} \u23CF \u23CF ")
        return chain
    else:
        LOG(" \u26D4 Request Forbidden \u26D4 ")
        exit()

def checkOptionPayouts(c):
    LOG("")
    address = input(f" \u25FB Input Stash Address (optional) (-a) | go to the  {c} Network & click Staking, then paste the address here  \u25B6 \u25B6 :")
    depth = input(" \u25FB Input era (-e) MAX: 84 \u25B6 : ")
    if int(depth)+1 > 10: 
        # MAX
        # LOG("84")
        LOG("Loading ...")
    LOG(" --- Fetching Data Payout Information --- ")
    request_params = {
        "depth": int(depth)
    }
    data = checkOptionSidecar(f'/accounts/{address}/staking-payouts', 'get', params=request_params)
    if ("code" in data and data["code"] == 400) or ("at" not in data or "height" not in data["at"]):
        LOG(f"{data['message']} \u26D4  Throw Error | Request Forbidden \u26D4 ")
        exit()
    total_payout = 0
    if "era (-e) payout" in data and len(data["era (-e) payout"]) > 0:
        for era in data["era (-e) payout"]:
            for ep in era["Payouts"]:
                if 'Nominator Staking Payout' in ep and 'Claimed' in ep and not ep['laimed']:
                    total_payout += int(ep["nominatorStakingPayout"])
    total_payout /= 10**u256
    total_payout = round(total_payout, 2)
    LOG(f" \u25FB Unclaimed Staking Payout  Address [ SS58 ] \u25B6 \u25B6 : {address}  \u25B6 : {total_payout} DOT at block \u25B6 : #{data['at']['height']} ")
    
if __name__ == "__main__":
    LOG(burgerSign)
    checkOpt2_Url()
    baseoption = checkOptionChain()
    checkOptionPayouts(baseoption)
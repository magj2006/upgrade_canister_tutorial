# upgrade_canister_tutorial

As you know it's hard to upgrade one canister in IC blockchain, I will performance how to upgrade one canister written by rust language step by step.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
# dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

v0.2: I will add a new age field to Student, and add new method set_age for setting age.

<img width="453" alt="截屏2022-07-28 11 03 31" src="https://user-images.githubusercontent.com/8394789/181411282-2d2f5d72-929b-4928-8988-d704be3244e6.png">


you can call greet for checking name you saved in last step and call set_age method to set age.

<img width="403" alt="截屏2022-07-28 11 04 25" src="https://user-images.githubusercontent.com/8394789/181411377-80f685fc-b89a-48f5-aa20-186290486373.png">





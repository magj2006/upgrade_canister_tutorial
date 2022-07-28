# upgrade_canister_tutorial

As you know it's hard to upgrade one canister in IC blockchain, I will performance how to upgrade one canister written by rust language step by step.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

v0.1: We have one Student struct, it only contains name field. and there are two method in canister, which one is greet for getting Student information and another is set_name for setting name of student.

<img width="662" alt="截屏2022-07-28 10 44 43" src="https://user-images.githubusercontent.com/8394789/181408974-a29774dd-32c1-4e0d-bc64-eb8d64e55ea5.png">

you can call set_name and greet then check output.

<img width="394" alt="截屏2022-07-28 10 45 53" src="https://user-images.githubusercontent.com/8394789/181409110-e6ba2585-9631-492a-a729-11512500b8c9.png">



# BlockStripe Smart Contract for NEAR

### Description
- Smart Contract written on Rust for NEAR blockchain for BlockStripe DeFi tool. BlockStripe allows users to execute their payments after certain period of time means everybody can config own payment execution job.
- Whitepaper available by link https://docs.google.com/document/d/1qegmx8MvZ_42zQwlqCQHlo6xGlxUFcZx/edit?usp=sharing&ouid=109225603762841589854&rtpof=true&sd=true.
- Current contract has been deployed only to `Testnet`, and here you can find it https://explorer.testnet.near.org/accounts/dev-1697308514888-93637017803912.

### Build and Run
- `./build.sh`

### Deploy
- `./deploy.sh`
- `./init.sh <contract_id> <trusted_account>` (where <trusted_account> is the one account you trust that can perform transfer() method and <contract_id> is a deployed contract address)

### Contract structure and methods

#### Getters
- `get_email_for_account`
- Description: Method that returns email for account.
- Arguments: `{ tenant_account_id: string }`
- `get_tenant_id_for_account`
- Description: Method that returns tenant id for account.
- Arguments: `{ tenant_account_id: string }`

#### Setters
- `add_tenant`
- Description: Method that adds a tenant for account.
- Arguments: `{ email: string }`
- `add_tenant_executable`
- Description: Method that add executable for account tenant.
- Arguments: `{ executable_counts: number, executable_amount: number, executable_recepient_account: string, executable_recepient_email: string }`
- `cancel_executable_early`
- Description: Method that allow to cancel executable early before term will be passed.
- Arguments: `{ tenant_executable_id: string }`
- `trigger_tenant_executable`
- Description: Method that triggers executable for account.
- Arguments: `{ tenant_executable_id: string }`

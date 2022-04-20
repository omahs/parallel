initSidebarItems({"constant":[["MAX_UNLOCKING_CHUNKS",""]],"enum":[["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","Custom dispatch errors of this pallet."],["Event","The event emitted by this pallet."],["Versions","Utility type for managing upgrades/migrations."]],"struct":[["GenesisConfig","Can be used to configure the genesis state of this pallet."],["Pallet","The pallet implementing the on-chain logic."],["_GeneratedPrefixForStorageCurrentEra",""],["_GeneratedPrefixForStorageEraStartBlock",""],["_GeneratedPrefixForStorageExchangeRate",""],["_GeneratedPrefixForStorageIsUpdated",""],["_GeneratedPrefixForStorageMatchingPool",""],["_GeneratedPrefixForStorageReserveFactor",""],["_GeneratedPrefixForStorageStakingLedgerCap",""],["_GeneratedPrefixForStorageStakingLedgers",""],["_GeneratedPrefixForStorageTotalReserves",""],["_GeneratedPrefixForStorageUnlockings",""],["_GeneratedPrefixForStorageValidationData",""],["_GeneratedPrefixForStorageXcmRequests",""]],"trait":[["Config","Configuration trait of this pallet."]],"type":[["AccountIdOf",""],["AssetIdOf",""],["BalanceOf",""],["CurrentEra","Current era index Users can come to claim their unbonded staking assets back once this value arrived at certain height decided by `BondingDuration` and `EraLength`"],["EraStartBlock","Current era’s start relaychain block"],["ExchangeRate","The exchange rate between relaychain native asset and the voucher."],["IsUpdated","Set to true if staking ledger has been modified in this block"],["MatchingPool","Store total stake amount and unstake amount in each era, And will update when stake/unstake occurred."],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["ReserveFactor","Fraction of reward currently set aside for reserves."],["StakingLedgerCap","Staking ledger’s cap"],["StakingLedgers","Platform’s staking ledgers"],["TotalReserves",""],["Unlockings","Unbonding requests to be handled after arriving at target era"],["ValidationData","ValidationData of previous block"],["XcmRequests","Flying & failed xcm requests"]]});
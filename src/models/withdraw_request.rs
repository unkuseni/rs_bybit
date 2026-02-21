use crate::prelude::*;

/// Request for creating a withdrawal
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawRequest<'a> {
    /// Coin, uppercase only
    pub coin: &'a str,

    /// Chain
    /// - `forceChain`=0 or 1: this field is **required**
    /// - `forceChain`=2: this field can be null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<&'a str>,

    /// Address
    /// - `forceChain`=0 or 1: fill wallet address, and make sure you add address in the address book first.
    ///   Please note that the address is case sensitive, so use the exact same address added in address book
    /// - `forceChain`=2: fill Bybit UID, and it can only be another Bybit **main** account UID.
    ///   Make sure you add UID in the address book first
    pub address: &'a str,

    /// Tag
    /// - **Required** if tag exists in the wallet address list.
    /// - **Note**: please do not set a tag/memo in the address book if the chain does not support tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<&'a str>,

    /// Withdraw amount
    pub amount: &'a str,

    /// Current timestamp (ms). Used for preventing from withdraw replay
    pub timestamp: u64,

    /// Whether or not to force an on-chain withdrawal
    /// - `0`(default): If the address is parsed out to be an internal address, then internal transfer (**Bybit main account only**)
    /// - `1`: Force the withdrawal to occur on-chain
    /// - `2`: Use UID to withdraw
    #[serde(rename = "forceChain", skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<i32>,

    /// Select the wallet to be withdrawn from
    /// - `FUND`: Funding wallet
    /// - `UTA`: System transfers the funds to Funding wallet to withdraw
    /// - `FUND,UTA`: For combo withdrawals, funds will be deducted from the Funding wallet first.
    ///   If the balance is insufficient, the remaining amount will be deducted from the UTA wallet.
    #[serde(rename = "accountType")]
    pub account_type: &'a str,

    /// Handling fee option
    /// - `0`(default): input amount is the actual amount received, so you have to calculate handling fee manually
    /// - `1`: input amount is not the actual amount you received, the system will help to deduct the handling fee automatically
    #[serde(rename = "feeType", skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<i32>,

    /// Customised ID, globally unique, it is used for idempotent verification
    /// - A combination of letters (case sensitive) and numbers, which can be pure letters or pure numbers
    ///   and the length must be between 1 and 32 digits
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<&'a str>,

    /// Travel rule info. It is required for kyc/kyb=KOR (Korean), kyc=IND (India) users,
    /// and users who registered in Bybit Turkey(TR), Bybit Kazakhstan(KZ), Bybit Indonesia (ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<BeneficiaryInfo<'a>>,
}

/// Travel rule beneficiary information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeneficiaryInfo<'a> {
    /// Purpose of the withdrawal transaction, Required when KR users withdraw funds to a company via Korean CODE channel
    #[serde(
        rename = "beneficiaryTransactionPurpose",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_transaction_purpose: Option<&'a str>,

    /// First name of the beneficiary company's representative, Required when KR users withdraw funds to a company via Korean CODE channel
    #[serde(
        rename = "beneficiaryRepresentativeFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_representative_first_name: Option<&'a str>,

    /// Last name of the beneficiary company's representative, Required when KR users withdraw funds to a company via Korean CODE channel
    #[serde(
        rename = "beneficiaryRepresentativeLastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_representative_last_name: Option<&'a str>,

    /// Receiver exchange entity Id. Please call this endpoint to get this ID.
    /// - Required param for Korean users
    /// - Ignored by TR, KZ users
    #[serde(rename = "vaspEntityId", skip_serializing_if = "Option::is_none")]
    pub vasp_entity_id: Option<&'a str>,

    /// Receiver exchange user KYC name
    /// Rules for Korean users:
    /// - Please refer to target exchange kyc name
    /// - When vaspEntityId="others", this field can be null
    /// Rules for TR, KZ, kyc=IND users: it is a required param, fill with individual name or company name
    #[serde(rename = "beneficiaryName", skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<&'a str>,

    /// Beneficiary legal type, `individual`(default), `company`
    /// - Required param for TR, KZ, kyc=IND users
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryLegalType",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_legal_type: Option<&'a str>,

    /// Beneficiary wallet type, `0`: custodial/exchange wallet (default), `1`: non custodial/exchange wallet
    /// - Required param for TR, KZ, kyc=IND users
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryWalletType",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_wallet_type: Option<&'a str>,

    /// Beneficiary unhosted wallet type, `0`: Your own wallet, `1`: others' wallet
    /// - Required param for TR, KZ, kyc=IND users when "beneficiaryWalletType=1"
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryUnhostedWalletType",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_unhosted_wallet_type: Option<&'a str>,

    /// Beneficiary document number
    /// - Required param for TR, KZ users
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryPoiNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_poi_number: Option<&'a str>,

    /// Beneficiary document type
    /// - Required param for TR, KZ users: ID card, Passport, driver license, residence permit, Business ID, etc
    /// - Korean users can ignore
    #[serde(rename = "beneficiaryPoiType", skip_serializing_if = "Option::is_none")]
    pub beneficiary_poi_type: Option<&'a str>,

    /// Beneficiary document issuing country
    /// - Required param for TR, KZ users: refer to Alpha-3 country code
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryPoiIssuingCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_poi_issuing_country: Option<&'a str>,

    /// Beneficiary document expiry date
    /// - Required param for TR, KZ users: yyyy-mm-dd format, e.g., "1990-02-15"
    /// - Korean users can ignore
    #[serde(
        rename = "beneficiaryPoiExpiredDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_poi_expired_date: Option<&'a str>,

    /// Beneficiary country
    /// - Required param for UAE users only, e.g.,`IDN`
    #[serde(
        rename = "beneficiaryAddressCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_country: Option<&'a str>,

    /// Beneficiary state
    /// - Required param for UAE users only, e.g., "ABC"
    #[serde(
        rename = "beneficiaryAddressState",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_state: Option<&'a str>,

    /// Beneficiary city
    /// - Required param for UAE users only, e.g., "Jakarta"
    #[serde(
        rename = "beneficiaryAddressCity",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_city: Option<&'a str>,

    /// Beneficiary building address
    /// - Required param for UAE users only
    #[serde(
        rename = "beneficiaryAddressBuilding",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_building: Option<&'a str>,

    /// Beneficiary street address
    /// - Required param for UAE users only
    #[serde(
        rename = "beneficiaryAddressStreet",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_street: Option<&'a str>,

    /// Beneficiary address post code
    /// - Required param for UAE users only
    #[serde(
        rename = "beneficiaryAddressPostalCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_address_postal_code: Option<&'a str>,

    /// Beneficiary date of birth
    /// - Required param for UAE users only
    #[serde(
        rename = "beneficiaryDateOfBirth",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_date_of_birth: Option<&'a str>,

    /// Beneficiary birth place
    /// - Required param for UAE users only
    #[serde(
        rename = "beneficiaryPlaceOfBirth",
        skip_serializing_if = "Option::is_none"
    )]
    pub beneficiary_place_of_birth: Option<&'a str>,
}

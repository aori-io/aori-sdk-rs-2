use jsonrpsee::{core::ClientError, types::ErrorObject};
use thiserror::Error;
use tracing::error;
use serde::Serialize;

// shoutout chatgpt ^^

#[derive(Error, Debug, Serialize)]
pub enum AoriBackendErrors {
    #[error("Your JSON-RPC payload data doesn't fit into the correct format")]
    IncorrectRequestFormat(),
    #[error("Other server error: {0}")]
    OtherError(String),
    #[error("Invalid Authorization")]
    InvalidAuthorization(),
    #[error("Invalid API Key")]
    InvalidAPIKey(),
    #[error("API Key Expired")]
    APIKeyExpired(),
    #[error("Unauthorised")]
    Unauthorised(),
    #[error("Internal Server Error")]
    InternalServerError(),
    #[error("Invalid Order Id")]
    InvalidOrderId(),
    #[error("Invalid Order Hash")]
    InvalidOrderHash(),
    #[error("Order no longer active")]
    OrderInactive(),
    #[error("Order already Cancelled")]
    OrderAlreadyCancelled(),
    #[error("Order's signature is invalid (corrupt or not offerer)")]
    OrderDoesntBelongToSigner(),
    #[error("Order has already been taken")]
    OrderAlreadyTaken(),
    #[error("Could not get timestamp to track block")]
    BlockTimestampNull(),
    #[error("Invalid Consideration Token")]
    InvalidConsiderationToken(),
    #[error("Invalid Offer Token")]
    InvalidOfferToken(),
    #[error("Invalid Chain Id")]
    InvalidChainId(),
    #[error("Aori Fee Not Included")]
    InvalidFeeConsideration(),
    #[error("Invalid Offer Amount")]
    InvalidOfferAmount(),
    #[error("Invalid Consideration Amount")]
    InvalidConsiderationAmount(),
    #[error("No Native Balance on Chain")]
    NoNativeBalance(),
    #[error("Missing Enough Maker Balance for Limit Order")]
    MissingMakerBalance(),
    #[error("Offer Token and Consideration Token are the same")]
    OfferTokenAndConsiderationTokensAreTheSame(),
    #[error("Offer Start Amount is zero")]
    OfferStartAmountIsZero(),
    #[error("Offer End Amount is zero")]
    OfferEndAmountIsZero(),
    #[error("Offer Start and End Amount aren't the same")]
    OfferStartAndEndAmountArentTheSame(),
    #[error("Consideration Start Amount is zero")]
    ConsiderationStartAmountIsZero(),
    #[error("Consideration End Amount is zero")]
    ConsiderationEndAmountIsZero(),
    #[error("Consideration Start and End Amount aren't the same")]
    ConsiderationStartAndEndAmountArentTheSame(),
    #[error("Could not compute order components")]
    CouldntComputeOrderComponents(),
    #[error("Could not compute order hash")]
    CouldntComputeOrderHash(),
    #[error("Could not get signers native balance")]
    CouldntGetSignersNativeBalance(),
    #[error("Could not get makers token balance")]
    CouldntGetMakersTokenBalance(),
    #[error("Failed to insert order")]
    FailedToInsertOrder(),
    #[error("Signature could not be verified")]
    SignatureCouldNotBeVerified(),

    // aori_takeOrder
    #[error("Taker order doesn't have signature")]
    TakerOrderDoesntHaveSignature(),
    #[error("Offer must have exactly one item")]
    OfferMustHaveExactlyOneItem(),
    #[error("Consideration must have exactly one item")]
    ConsiderationMustHaveExactlyOneItem(),
    #[error("Order must be partial restricted")]
    OrderMustBePartialRestricted(),
    #[error("startTime is incorrectly bigger than endTime")]
    StartTimeIsIncorrectlyBiggerThanEndTime(),
    #[error("Order has already expired")]
    OrderHasAlreadyExpired(),
    #[error("Order has invalid consideration item length")]
    OrderHasInvalidConsiderationItemLength(),
    #[error("Signature is invalid")]
    SignatureInvalid(),
    #[error("Maker and taker must be different")]
    MakerAndTakerMustBeDifferent(),
    #[error("Taker order and maker order must be in the same zone")]
    TakerOrderAndMakerOrderMustBeInSameZone(),
    #[error("Taker offer token and maker consideration token aren't the same")]
    TakerOfferTokenAndMakerConsiderationTokenArentTheSame(),
    #[error("Taker consideration token and maker offer token aren't the same")]
    TakerConsiderationAndMakerOfferTokenArentTheSame(),

    //
    #[error("Taker order has insufficient assets")]
    TakerOrderHasInsufficientAssets(),
    #[error("Could not get taker balance")]
    CouldNotGetTakerBalance(),
    #[error("Could not get taker allowance")]
    CouldNotGetTakerAllowance(),
    #[error("Maker order being fulfilled by another")]
    MakerOrderBeingFulfilledByAnother(),

    #[error("Currently Only Supporting ERC20s")]
    CurrentlyOnlySupportingERC20s(),
    #[error("Bad Identifier or Criteria")]
    BadIdentifierOrCriteria(),
    #[error("Unsupported zone")]
    UnsupportedZone(),
    #[error("Please stick to setting zoneHash as `0x0000000000000000000000000000000000000000000000000000000000000000`")]
    EsotericZoneHash(),
    #[error("Failed to validate order")]
    FailedToValidateOrder(),
    #[error("Failed to update order")]
    FailedToUpdateOrder(),
    #[error("Maker and taker order must be on the same chain")]
    MakerAndTakerOrderMustBeOnTheSameChain(),

    #[error("Missing input amount or output amount")]
    MissingInputAmountOrOutputAmount(),
    #[error("Invalid input amount")]
    InvalidInputAmount(),
    #[error("Invalid output amount")]
    InvalidOutputAmount(),
    #[error("Failed to fetch API Key")]
    FailedToFetchApiKey(),

    // aori_apiKey
    #[error("Failed to insert generated API Key")]
    FailedToInsertApiKey(),
    #[error("API key with email already exists")]
    APIKeyAlreadyExists(),
    #[error("Taker has not included enough for fees")]
    TakerHasNotIncludedEnoughForFees(),
}

impl AoriBackendErrors {
    fn from_error_message(message: &str) -> Self {
        match message {
            "Your JSON-RPC payload data doesn't fit into the correct format" => AoriBackendErrors::IncorrectRequestFormat(),
            "Invalid Authorization" => AoriBackendErrors::InvalidAuthorization(),
            "Invalid API Key" => AoriBackendErrors::InvalidAPIKey(),
            "API Key Expired" => AoriBackendErrors::APIKeyExpired(),
            "Unauthorised" => AoriBackendErrors::Unauthorised(),
            "Internal Server Error" => AoriBackendErrors::InternalServerError(),
            "Invalid Order Id" => AoriBackendErrors::InvalidOrderId(),
            "Invalid Order Hash" => AoriBackendErrors::InvalidOrderHash(),
            "Order no longer active" => AoriBackendErrors::OrderInactive(),
            "Order already Cancelled" => AoriBackendErrors::OrderAlreadyCancelled(),
            "Order's signature is invalid (corrupt or not offerer)" => AoriBackendErrors::OrderDoesntBelongToSigner(),
            "Order has already been taken" => AoriBackendErrors::OrderAlreadyTaken(),
            "Could not get timestamp to track block" => AoriBackendErrors::BlockTimestampNull(),
            "Invalid Consideration Token" => AoriBackendErrors::InvalidConsiderationToken(),
            "Invalid Offer Token" => AoriBackendErrors::InvalidOfferToken(),
            "Invalid Chain Id" => AoriBackendErrors::InvalidChainId(),
            "Aori Fee Not Included" => AoriBackendErrors::InvalidFeeConsideration(),
            "Invalid Offer Amount" => AoriBackendErrors::InvalidOfferAmount(),
            "Invalid Consideration Amount" => AoriBackendErrors::InvalidConsiderationAmount(),
            "No Native Balance on Chain" => AoriBackendErrors::NoNativeBalance(),
            "Missing Enough Maker Balance for Limit Order" => AoriBackendErrors::MissingMakerBalance(),
            "Offer Token and Consideration Token are the same" => AoriBackendErrors::OfferTokenAndConsiderationTokensAreTheSame(),
            "Offer Start Amount is zero" => AoriBackendErrors::OfferStartAmountIsZero(),
            "Offer End Amount is zero" => AoriBackendErrors::OfferEndAmountIsZero(),
            "Offer Start and End Amount aren't the same" => AoriBackendErrors::OfferStartAndEndAmountArentTheSame(),
            "Consideration Start Amount is zero" => AoriBackendErrors::ConsiderationStartAmountIsZero(),
            "Consideration End Amount is zero" => AoriBackendErrors::ConsiderationEndAmountIsZero(),
            "Consideration Start and End Amount aren't the same" => AoriBackendErrors::ConsiderationStartAndEndAmountArentTheSame(),
            "Could not compute order components" => AoriBackendErrors::CouldntComputeOrderComponents(),
            "Could not compute order hash" => AoriBackendErrors::CouldntComputeOrderHash(),
            "Could not get signers native balance" => AoriBackendErrors::CouldntGetSignersNativeBalance(),
            "Could not get makers token balance" => AoriBackendErrors::CouldntGetMakersTokenBalance(),
            "Failed to insert order" => AoriBackendErrors::FailedToInsertOrder(),
            "Signature could not be verified" => AoriBackendErrors::SignatureCouldNotBeVerified(),
            "Taker order doesn't have signature" => AoriBackendErrors::TakerOrderDoesntHaveSignature(),
            "Offer must have exactly one item" => AoriBackendErrors::OfferMustHaveExactlyOneItem(),
            "Consideration must have exactly one item" => AoriBackendErrors::ConsiderationMustHaveExactlyOneItem(),
            "Order must be partial restricted" => AoriBackendErrors::OrderMustBePartialRestricted(),
            "startTime is incorrectly bigger than endTime" => AoriBackendErrors::StartTimeIsIncorrectlyBiggerThanEndTime(),
            "Order has already expired" => AoriBackendErrors::OrderHasAlreadyExpired(),
            "Order has invalid consideration item length" => AoriBackendErrors::OrderHasInvalidConsiderationItemLength(),
            "Signature is invalid" => AoriBackendErrors::SignatureInvalid(),
            "Maker and taker must be different" => AoriBackendErrors::MakerAndTakerMustBeDifferent(),
            "Taker order and maker order must be in the same zone" => AoriBackendErrors::TakerOrderAndMakerOrderMustBeInSameZone(),
            "Taker offer token and maker consideration token aren't the same" => AoriBackendErrors::TakerOfferTokenAndMakerConsiderationTokenArentTheSame(),
            "Taker consideration token and maker offer token aren't the same" => AoriBackendErrors::TakerConsiderationAndMakerOfferTokenArentTheSame(),
            "Currently Only Supporting ERC20s" => AoriBackendErrors::CurrentlyOnlySupportingERC20s(),
            "Bad Identifier or Criteria" => AoriBackendErrors::BadIdentifierOrCriteria(),
            "Unsupported zone" => AoriBackendErrors::UnsupportedZone(),
            "Please stick to setting zoneHash as `0x0000000000000000000000000000000000000000000000000000000000000000`" => AoriBackendErrors::EsotericZoneHash(),
            "Failed to validate order" => AoriBackendErrors::FailedToValidateOrder(),
            "Failed to update order" => AoriBackendErrors::FailedToUpdateOrder(),
            "Maker and taker order must be on the same chain" => AoriBackendErrors::MakerAndTakerOrderMustBeOnTheSameChain(),
            "Missing input amount or output amount" => AoriBackendErrors::MissingInputAmountOrOutputAmount(),
            "Invalid input amount" => AoriBackendErrors::InvalidInputAmount(),
            "Invalid output amount" => AoriBackendErrors::InvalidOutputAmount(),
            "Failed to fetch API Key" => AoriBackendErrors::FailedToFetchApiKey(),
            "Failed to insert generated API Key" => AoriBackendErrors::FailedToInsertApiKey(),
            "API key with email already exists" => AoriBackendErrors::APIKeyAlreadyExists(),
            "Taker has not included enough for fees" => AoriBackendErrors::TakerHasNotIncludedEnoughForFees(),
            _ => AoriBackendErrors::OtherError(message.to_owned()),
        }
    }
}

impl From<ErrorObject<'_>> for AoriBackendErrors {
    fn from(e: ErrorObject) -> Self {
        AoriBackendErrors::from_error_message(e.message())
    }
}

impl From<ClientError> for AoriBackendErrors {
    fn from(e: ClientError) -> Self {
        match e {
            ClientError::Call(err) => AoriBackendErrors::from_error_message(err.message()),
            // Handle other variants of `ClientError` as needed
            _ => {
                // For unhandled cases, you might want to log the error and create a default
                // `MyRpcServerError` or handle it in another appropriate way
                // according to your application's error handling policy.
                AoriBackendErrors::OtherError(format!("Unhandled server error: {:?}", e))
            }
        }
    }
}

use crate::prelude::*;

/// Enum representing different types of batch order requests.
///
/// This enum allows bots to specify whether a batch request is for creating, amending, or canceling orders.
/// It provides flexibility for handling multiple order operations in a unified way.
#[derive(Clone)]
pub enum RequestType<'a> {
    /// A batch order creation request.
    ///
    /// Used to place multiple new orders. Bots use this for strategies requiring simultaneous order placements,
    /// such as grid trading or scalping.
    Create(OrderRequest<'a>),

    /// A batch order creation request for multiple orders.
    ///
    /// Used to place multiple new orders in a single request. Bots use this for strategies requiring simultaneous order placements,
    /// such as grid trading or scalping.
    CreateBatch(BatchPlaceRequest<'a>),

    /// A batch order amendment request.
    ///
    /// Used to modify multiple existing orders. Bots use this to adjust order parameters in response to market changes,
    /// such as adjusting stop-losses or take-profits.
    Amend(AmendOrderRequest<'a>),

    /// A batch order amendment request for multiple orders.
    ///
    /// Used to modify multiple existing orders in a single request. Bots use this to adjust order parameters in response to market changes,
    /// such as adjusting stop-losses or take-profits.
    AmendBatch(BatchAmendRequest<'a>),

    /// A batch order cancellation request.
    ///
    /// Used to cancel multiple existing orders. Bots use this for risk management or strategy adjustments,
    /// such as canceling orders when a position is closed or when a strategy is changed.
    Cancel(CancelOrderRequest<'a>),

    /// A batch order cancellation request for multiple orders.
    ///
    /// Used to cancel multiple existing orders in a single request. Bots use this for risk management or strategy adjustments,
    /// such as canceling orders when a position is closed or when a strategy is changed.
    CancelBatch(BatchCancelRequest<'a>),

    /// A subscription request.
    ///
    /// Used to subscribe to real-time updates for a set of topics. Bots use this to receive notifications about market data or order status changes.
    Subscribe(Subscription<'a>),

    /// An unsubscribe request.
    ///
    /// Used to unsubscribe from real-time updates for a set of topics. Bots use this to stop receiving notifications about market data or order status changes.
    Unsubscribe(Subscription<'a>),
}

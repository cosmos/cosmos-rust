use crate::proto;

/// PageRequest is to be embedded in gRPC request messages for efficient
/// pagination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Vec<u8>,

    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,

    /// reverse is set to true if results are to be returned in the descending order.
    ///
    /// Since: cosmos-sdk 0.43
    pub reverse: bool,
}

impl From<proto::cosmos::base::query::v1beta1::PageRequest> for PageRequest {
    fn from(proto: cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest) -> Self {
        PageRequest {
            key: proto.key,
            offset: proto.offset,
            limit: proto.limit,
            count_total: proto.count_total,
            reverse: proto.reverse,
        }
    }
}

impl From<PageRequest> for proto::cosmos::base::query::v1beta1::PageRequest {
    fn from(page_request: PageRequest) -> Self {
        proto::cosmos::base::query::v1beta1::PageRequest {
            key: page_request.key,
            offset: page_request.offset,
            limit: page_request.limit,
            count_total: page_request.count_total,
            reverse: page_request.reverse,
        }
    }
}

/// PageResponse is to be embedded in gRPC response messages where the
/// corresponding request message has used PageRequest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageResponse {
    /// next_key is the key to be passed to PageRequest.key to
    /// query the next page most efficiently. It will be empty if
    /// there are no more results.
    pub next_key: Vec<u8>,

    /// total is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    pub total: u64,
}

impl From<proto::cosmos::base::query::v1beta1::PageResponse> for PageResponse {
    fn from(proto: cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse) -> Self {
        PageResponse {
            next_key: proto.next_key,
            total: proto.total,
        }
    }
}

impl From<PageResponse> for proto::cosmos::base::query::v1beta1::PageResponse {
    fn from(page_response: PageResponse) -> Self {
        proto::cosmos::base::query::v1beta1::PageResponse {
            next_key: page_response.next_key,
            total: page_response.total,
        }
    }
}

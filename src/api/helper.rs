use crate::api::dto::content::{FriendsBlockOpts, FriendsFollowOpts, FriendsHopsOpts,
                               FriendsIsBlockingOpts, FriendsIsFollowingOpts, InviteCreateOpts};
use crate::{
    api::dto::content::{SubsetQuery, SubsetQueryOptions, TypedMessage},
    feed::Message,
    rpc::{Body, BodyType, RequestNo, RpcType, RpcWriter},
};
use async_std::io::Write;

use super::{dto, error::Result};

const MAX_RPC_BODY_LEN: usize = 65536;

#[derive(Debug)]
pub enum ApiMethod {
    GetSubset,
    Publish,
    FriendsFollow,
    FriendsBlock,
    FriendsIsFollowing,
    FriendsIsBlocking,
    FriendsHops,
    InviteCreate,
    InviteUse,
    WhoAmI,
    Get,
    CreateHistoryStream,
    CreateFeedStream,
    Latest,
    BlobsGet,
    BlobsCreateWants,
}

impl ApiMethod {
    pub fn selector(&self) -> &'static [&'static str] {
        use ApiMethod::*;
        match self {
            GetSubset => &["partialReplication", "getSubset"],
            Publish => &["publish"],
            FriendsFollow => &["friends", "follow"],
            FriendsBlock => &["friends", "block"],
            FriendsIsFollowing => &["friends", "isFollowing"],
            FriendsIsBlocking => &["friends", "isBlocking"],
            FriendsHops => &["friends", "hops"],
            InviteCreate => &["invite", "create"],
            InviteUse => &["invite", "use"],
            WhoAmI => &["whoami"],
            Get => &["get"],
            CreateHistoryStream => &["createHistoryStream"],
            CreateFeedStream => &["createFeedStream"],
            Latest => &["latest"],
            BlobsGet => &["blobs", "get"],
            BlobsCreateWants => &["blobs", "createWants"],
        }
    }
    pub fn from_selector(s: &[&str]) -> Option<Self> {
        use ApiMethod::*;
        match s {
            ["partialReplication", "getSubset"] => Some(GetSubset),
            ["publish"] => Some(Publish),
            ["friends", "follow"] => Some(FriendsFollow),
            ["friends", "block"] => Some(FriendsBlock),
            ["friends", "isFollowing"] => Some(FriendsIsFollowing),
            ["friends", "isBlocking"] => Some(FriendsIsBlocking),
            ["friends", "hops"] => Some(FriendsHops),
            ["invite", "create"] => Some(InviteCreate),
            ["invite", "use"] => Some(InviteUse),
            ["whoami"] => Some(WhoAmI),
            ["get"] => Some(Get),
            ["createHistoryStream"] => Some(CreateHistoryStream),
            ["createFeedStream"] => Some(CreateFeedStream),
            ["latest"] => Some(Latest),
            ["blobs", "get"] => Some(BlobsGet),
            ["blobs", "createWants"] => Some(BlobsCreateWants),
            _ => None,
        }
    }
    pub fn from_rpc_body(body: &Body) -> Option<Self> {
        let selector = body.name.iter().map(|v| v.as_str()).collect::<Vec<_>>();
        Self::from_selector(&selector)
    }
}

pub struct ApiCaller<W: Write + Unpin> {
    rpc: RpcWriter<W>,
}

impl<W: Write + Unpin> ApiCaller<W> {
    pub fn new(rpc: RpcWriter<W>) -> Self {
        Self { rpc }
    }

    pub fn rpc(&mut self) -> &mut RpcWriter<W> {
        &mut self.rpc
    }

    /// Send ["partialReplication", "getSubset"] request.
    pub async fn getsubset_req_send(
        &mut self,
        query: SubsetQuery,
        opts: Option<SubsetQueryOptions>,
    ) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::GetSubset.selector(),
                RpcType::Source,
                &query,
                &opts,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["publish"] request.
    pub async fn publish_req_send(&mut self, msg: TypedMessage) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::Publish.selector(),
                RpcType::Async,
                &msg,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["publish"] response.
    pub async fn publish_res_send(&mut self, req_no: RequestNo, msg_ref: String) -> Result<()> {
        Ok(self
            .rpc
            .send_response(req_no, RpcType::Async, BodyType::JSON, msg_ref.as_bytes())
            .await?)
    }

    /// Send ["friends", "block"] request.
    pub async fn friends_block_req_send(&mut self) -> Result<RequestNo> {
        let args: [&str; 0] = []; // todo: add this
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::FriendsBlock.selector(),
                RpcType::Async,
                &args,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["friends", "isfollowing"] request.
    pub async fn friends_isfollowing_req_send(
        &mut self,
        src_id: &str,
        dest_id: &str,
    ) -> Result<RequestNo> {
        let args = FriendsIsFollowingOpts {
            source: src_id.to_string(),
            dest: dest_id.to_string(),
        };
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::FriendsIsFollowing.selector(),
                RpcType::Async,
                &args,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["friends", "isblocking"] request.
    pub async fn friends_isblocking_req_send(
        &mut self,
        src_id: &str,
        dest_id: &str,
    ) -> Result<RequestNo> {
        let args = FriendsIsBlockingOpts {
            source: src_id.to_string(),
            dest: dest_id.to_string(),
        };
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::FriendsIsBlocking.selector(),
                RpcType::Async,
                &args,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["friends", "follow"] request.
    /// currently not working
    pub async fn friends_follow_req_send(
        &mut self,
        ssb_id: &str,
        state: bool,
    ) -> Result<RequestNo> {
        let args: [&str; 1] = [ssb_id];
        let opts = FriendsFollowOpts { state };
        // TODO: not sure how args and opts should be passed for this function
        // currently gets error:
        // Application error: Sbot returned an error response: muxrpc: no such command: friends.follow
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::FriendsFollow.selector(),
                RpcType::Async,
                &args,
                &Some(opts),
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["friends", "hops"] request
    pub async fn friends_hops_req_send(&mut self, opts: FriendsHopsOpts) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::FriendsHops.selector(),
                RpcType::Source,
                &opts,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["invite", "create"] request.
    pub async fn invite_create_req_send(
        &mut self,
        uses: i32,
    ) -> Result<RequestNo> {
        let args = InviteCreateOpts {
            uses,
        };
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::InviteCreate.selector(),
                RpcType::Async,
                &args,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["invite", "use"] request.
    pub async fn invite_use_req_send(
        &mut self,
        invite_link: &str,
    ) -> Result<RequestNo> {
        let args: [&str; 1] = [invite_link];
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::InviteUse.selector(),
                RpcType::Async,
                &args,
                // specify None value for `opts`
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["whoami"] request.
    pub async fn whoami_req_send(&mut self) -> Result<RequestNo> {
        let args: [&str; 0] = [];
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::WhoAmI.selector(),
                RpcType::Async,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["whoami"] response.
    pub async fn whoami_res_send(&mut self, req_no: RequestNo, id: String) -> Result<()> {
        let body = serde_json::to_string(&dto::WhoAmIOut { id })?;
        Ok(self
            .rpc
            .send_response(req_no, RpcType::Async, BodyType::JSON, body.as_bytes())
            .await?)
    }

    /// Send ["get"] request.
    pub async fn get_req_send(&mut self, msg_id: &str) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::Get.selector(),
                RpcType::Async,
                &msg_id,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["get"] response.
    pub async fn get_res_send(&mut self, req_no: RequestNo, msg: &Message) -> Result<()> {
        self.rpc
            .send_response(
                req_no,
                RpcType::Async,
                BodyType::JSON,
                msg.to_string().as_bytes(),
            )
            .await?;
        Ok(())
    }

    /// Send ["createHistoryStream"] request.
    pub async fn create_history_stream_req_send(
        &mut self,
        args: &dto::CreateHistoryStreamIn,
    ) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::CreateHistoryStream.selector(),
                RpcType::Source,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["createFeedStream"] request.
    pub async fn create_feed_stream_req_send<'a>(
        &mut self,
        args: &dto::CreateStreamIn<u64>,
    ) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::CreateFeedStream.selector(),
                RpcType::Source,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["latest"] request.
    pub async fn latest_req_send(&mut self) -> Result<RequestNo> {
        let args: [&str; 0] = [];
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::Latest.selector(),
                RpcType::Async,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send ["blobs","get"] request.
    pub async fn blobs_get_req_send(&mut self, args: &dto::BlobsGetIn) -> Result<RequestNo> {
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::BlobsGet.selector(),
                RpcType::Source,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send feed response
    pub async fn feed_res_send(&mut self, req_no: RequestNo, feed: &str) -> Result<()> {
        self.rpc
            .send_response(req_no, RpcType::Source, BodyType::JSON, feed.as_bytes())
            .await?;
        Ok(())
    }

    /// Send blob create wants
    pub async fn blob_create_wants_req_send(&mut self) -> Result<RequestNo> {
        let args: [&str; 0] = [];
        let req_no = self
            .rpc
            .send_request(
                ApiMethod::BlobsCreateWants.selector(),
                RpcType::Source,
                &args,
                &None::<()>,
            )
            .await?;
        Ok(req_no)
    }

    /// Send blob response
    pub async fn blobs_get_res_send<D: AsRef<[u8]>>(
        &mut self,
        req_no: RequestNo,
        data: D,
    ) -> Result<()> {
        let mut offset = 0;
        let data = data.as_ref();
        while offset < data.len() {
            let limit = std::cmp::min(data.len(), offset + MAX_RPC_BODY_LEN);
            self.rpc
                .send_response(
                    req_no,
                    RpcType::Source,
                    BodyType::Binary,
                    &data[offset..limit],
                )
                .await?;
            offset += MAX_RPC_BODY_LEN;
        }
        self.rpc.send_stream_eof(req_no).await?;
        Ok(())
    }
}

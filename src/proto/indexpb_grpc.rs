// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_INDEX_RAFT: ::grpcio::Method<super::eraftpb::Message, super::indexrpcpb::RaftDone> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Raft",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_RAFT_CONF_CHANGE: ::grpcio::Method<super::indexrpcpb::ConfChangeReq, super::indexrpcpb::RaftDone> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/RaftConfChange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_PROBE: ::grpcio::Method<super::indexrpcpb::ProbeReq, super::indexrpcpb::ProbeResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Probe",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_PEERS: ::grpcio::Method<super::indexrpcpb::PeersReq, super::indexrpcpb::PeersResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Peers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_METRICS: ::grpcio::Method<super::indexrpcpb::MetricsReq, super::indexrpcpb::MetricsResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Metrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_GET: ::grpcio::Method<super::indexrpcpb::GetReq, super::indexrpcpb::GetResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_PUT: ::grpcio::Method<super::indexrpcpb::PutReq, super::indexrpcpb::PutResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_DELETE: ::grpcio::Method<super::indexrpcpb::DeleteReq, super::indexrpcpb::DeleteResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_BULK_PUT: ::grpcio::Method<super::indexrpcpb::BulkPutReq, super::indexrpcpb::BulkPutResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/BulkPut",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_BULK_DELETE: ::grpcio::Method<super::indexrpcpb::BulkDeleteReq, super::indexrpcpb::BulkDeleteResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/BulkDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_COMMIT: ::grpcio::Method<super::indexrpcpb::CommitReq, super::indexrpcpb::CommitResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Commit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_ROLLBACK: ::grpcio::Method<super::indexrpcpb::RollbackReq, super::indexrpcpb::RollbackResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Rollback",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_MERGE: ::grpcio::Method<super::indexrpcpb::MergeReq, super::indexrpcpb::MergeResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Merge",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_SEARCH: ::grpcio::Method<super::indexrpcpb::SearchReq, super::indexrpcpb::SearchResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Search",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDEX_SCHEMA: ::grpcio::Method<super::indexrpcpb::SchemaReq, super::indexrpcpb::SchemaResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/indexpb.Index/Schema",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct IndexClient {
    client: ::grpcio::Client,
}

impl IndexClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        IndexClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn raft_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::RaftDone> {
        self.client.unary_call(&METHOD_INDEX_RAFT, req, opt)
    }

    pub fn raft(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<super::indexrpcpb::RaftDone> {
        self.raft_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_async_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RaftDone>> {
        self.client.unary_call_async(&METHOD_INDEX_RAFT, req, opt)
    }

    pub fn raft_async(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RaftDone>> {
        self.raft_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_conf_change_opt(&self, req: &super::indexrpcpb::ConfChangeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::RaftDone> {
        self.client.unary_call(&METHOD_INDEX_RAFT_CONF_CHANGE, req, opt)
    }

    pub fn raft_conf_change(&self, req: &super::indexrpcpb::ConfChangeReq) -> ::grpcio::Result<super::indexrpcpb::RaftDone> {
        self.raft_conf_change_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_conf_change_async_opt(&self, req: &super::indexrpcpb::ConfChangeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RaftDone>> {
        self.client.unary_call_async(&METHOD_INDEX_RAFT_CONF_CHANGE, req, opt)
    }

    pub fn raft_conf_change_async(&self, req: &super::indexrpcpb::ConfChangeReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RaftDone>> {
        self.raft_conf_change_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn probe_opt(&self, req: &super::indexrpcpb::ProbeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::ProbeResp> {
        self.client.unary_call(&METHOD_INDEX_PROBE, req, opt)
    }

    pub fn probe(&self, req: &super::indexrpcpb::ProbeReq) -> ::grpcio::Result<super::indexrpcpb::ProbeResp> {
        self.probe_opt(req, ::grpcio::CallOption::default())
    }

    pub fn probe_async_opt(&self, req: &super::indexrpcpb::ProbeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::ProbeResp>> {
        self.client.unary_call_async(&METHOD_INDEX_PROBE, req, opt)
    }

    pub fn probe_async(&self, req: &super::indexrpcpb::ProbeReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::ProbeResp>> {
        self.probe_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn peers_opt(&self, req: &super::indexrpcpb::PeersReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::PeersResp> {
        self.client.unary_call(&METHOD_INDEX_PEERS, req, opt)
    }

    pub fn peers(&self, req: &super::indexrpcpb::PeersReq) -> ::grpcio::Result<super::indexrpcpb::PeersResp> {
        self.peers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn peers_async_opt(&self, req: &super::indexrpcpb::PeersReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::PeersResp>> {
        self.client.unary_call_async(&METHOD_INDEX_PEERS, req, opt)
    }

    pub fn peers_async(&self, req: &super::indexrpcpb::PeersReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::PeersResp>> {
        self.peers_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn metrics_opt(&self, req: &super::indexrpcpb::MetricsReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::MetricsResp> {
        self.client.unary_call(&METHOD_INDEX_METRICS, req, opt)
    }

    pub fn metrics(&self, req: &super::indexrpcpb::MetricsReq) -> ::grpcio::Result<super::indexrpcpb::MetricsResp> {
        self.metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn metrics_async_opt(&self, req: &super::indexrpcpb::MetricsReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::MetricsResp>> {
        self.client.unary_call_async(&METHOD_INDEX_METRICS, req, opt)
    }

    pub fn metrics_async(&self, req: &super::indexrpcpb::MetricsReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::MetricsResp>> {
        self.metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_opt(&self, req: &super::indexrpcpb::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::GetResp> {
        self.client.unary_call(&METHOD_INDEX_GET, req, opt)
    }

    pub fn get(&self, req: &super::indexrpcpb::GetReq) -> ::grpcio::Result<super::indexrpcpb::GetResp> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::indexrpcpb::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::GetResp>> {
        self.client.unary_call_async(&METHOD_INDEX_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::indexrpcpb::GetReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::GetResp>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::indexrpcpb::PutReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::PutResp> {
        self.client.unary_call(&METHOD_INDEX_PUT, req, opt)
    }

    pub fn put(&self, req: &super::indexrpcpb::PutReq) -> ::grpcio::Result<super::indexrpcpb::PutResp> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::indexrpcpb::PutReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::PutResp>> {
        self.client.unary_call_async(&METHOD_INDEX_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::indexrpcpb::PutReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::PutResp>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::indexrpcpb::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::DeleteResp> {
        self.client.unary_call(&METHOD_INDEX_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::indexrpcpb::DeleteReq) -> ::grpcio::Result<super::indexrpcpb::DeleteResp> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::indexrpcpb::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::DeleteResp>> {
        self.client.unary_call_async(&METHOD_INDEX_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::indexrpcpb::DeleteReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::DeleteResp>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_put_opt(&self, req: &super::indexrpcpb::BulkPutReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::BulkPutResp> {
        self.client.unary_call(&METHOD_INDEX_BULK_PUT, req, opt)
    }

    pub fn bulk_put(&self, req: &super::indexrpcpb::BulkPutReq) -> ::grpcio::Result<super::indexrpcpb::BulkPutResp> {
        self.bulk_put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_put_async_opt(&self, req: &super::indexrpcpb::BulkPutReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::BulkPutResp>> {
        self.client.unary_call_async(&METHOD_INDEX_BULK_PUT, req, opt)
    }

    pub fn bulk_put_async(&self, req: &super::indexrpcpb::BulkPutReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::BulkPutResp>> {
        self.bulk_put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_delete_opt(&self, req: &super::indexrpcpb::BulkDeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::BulkDeleteResp> {
        self.client.unary_call(&METHOD_INDEX_BULK_DELETE, req, opt)
    }

    pub fn bulk_delete(&self, req: &super::indexrpcpb::BulkDeleteReq) -> ::grpcio::Result<super::indexrpcpb::BulkDeleteResp> {
        self.bulk_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bulk_delete_async_opt(&self, req: &super::indexrpcpb::BulkDeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::BulkDeleteResp>> {
        self.client.unary_call_async(&METHOD_INDEX_BULK_DELETE, req, opt)
    }

    pub fn bulk_delete_async(&self, req: &super::indexrpcpb::BulkDeleteReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::BulkDeleteResp>> {
        self.bulk_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_opt(&self, req: &super::indexrpcpb::CommitReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::CommitResp> {
        self.client.unary_call(&METHOD_INDEX_COMMIT, req, opt)
    }

    pub fn commit(&self, req: &super::indexrpcpb::CommitReq) -> ::grpcio::Result<super::indexrpcpb::CommitResp> {
        self.commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn commit_async_opt(&self, req: &super::indexrpcpb::CommitReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::CommitResp>> {
        self.client.unary_call_async(&METHOD_INDEX_COMMIT, req, opt)
    }

    pub fn commit_async(&self, req: &super::indexrpcpb::CommitReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::CommitResp>> {
        self.commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rollback_opt(&self, req: &super::indexrpcpb::RollbackReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::RollbackResp> {
        self.client.unary_call(&METHOD_INDEX_ROLLBACK, req, opt)
    }

    pub fn rollback(&self, req: &super::indexrpcpb::RollbackReq) -> ::grpcio::Result<super::indexrpcpb::RollbackResp> {
        self.rollback_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rollback_async_opt(&self, req: &super::indexrpcpb::RollbackReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RollbackResp>> {
        self.client.unary_call_async(&METHOD_INDEX_ROLLBACK, req, opt)
    }

    pub fn rollback_async(&self, req: &super::indexrpcpb::RollbackReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::RollbackResp>> {
        self.rollback_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn merge_opt(&self, req: &super::indexrpcpb::MergeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::MergeResp> {
        self.client.unary_call(&METHOD_INDEX_MERGE, req, opt)
    }

    pub fn merge(&self, req: &super::indexrpcpb::MergeReq) -> ::grpcio::Result<super::indexrpcpb::MergeResp> {
        self.merge_opt(req, ::grpcio::CallOption::default())
    }

    pub fn merge_async_opt(&self, req: &super::indexrpcpb::MergeReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::MergeResp>> {
        self.client.unary_call_async(&METHOD_INDEX_MERGE, req, opt)
    }

    pub fn merge_async(&self, req: &super::indexrpcpb::MergeReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::MergeResp>> {
        self.merge_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_opt(&self, req: &super::indexrpcpb::SearchReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::SearchResp> {
        self.client.unary_call(&METHOD_INDEX_SEARCH, req, opt)
    }

    pub fn search(&self, req: &super::indexrpcpb::SearchReq) -> ::grpcio::Result<super::indexrpcpb::SearchResp> {
        self.search_opt(req, ::grpcio::CallOption::default())
    }

    pub fn search_async_opt(&self, req: &super::indexrpcpb::SearchReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::SearchResp>> {
        self.client.unary_call_async(&METHOD_INDEX_SEARCH, req, opt)
    }

    pub fn search_async(&self, req: &super::indexrpcpb::SearchReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::SearchResp>> {
        self.search_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn schema_opt(&self, req: &super::indexrpcpb::SchemaReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::indexrpcpb::SchemaResp> {
        self.client.unary_call(&METHOD_INDEX_SCHEMA, req, opt)
    }

    pub fn schema(&self, req: &super::indexrpcpb::SchemaReq) -> ::grpcio::Result<super::indexrpcpb::SchemaResp> {
        self.schema_opt(req, ::grpcio::CallOption::default())
    }

    pub fn schema_async_opt(&self, req: &super::indexrpcpb::SchemaReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::SchemaResp>> {
        self.client.unary_call_async(&METHOD_INDEX_SCHEMA, req, opt)
    }

    pub fn schema_async(&self, req: &super::indexrpcpb::SchemaReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::indexrpcpb::SchemaResp>> {
        self.schema_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Index {
    fn raft(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::Message, sink: ::grpcio::UnarySink<super::indexrpcpb::RaftDone>);
    fn raft_conf_change(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::ConfChangeReq, sink: ::grpcio::UnarySink<super::indexrpcpb::RaftDone>);
    fn probe(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::ProbeReq, sink: ::grpcio::UnarySink<super::indexrpcpb::ProbeResp>);
    fn peers(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::PeersReq, sink: ::grpcio::UnarySink<super::indexrpcpb::PeersResp>);
    fn metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::MetricsReq, sink: ::grpcio::UnarySink<super::indexrpcpb::MetricsResp>);
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::GetReq, sink: ::grpcio::UnarySink<super::indexrpcpb::GetResp>);
    fn put(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::PutReq, sink: ::grpcio::UnarySink<super::indexrpcpb::PutResp>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::DeleteReq, sink: ::grpcio::UnarySink<super::indexrpcpb::DeleteResp>);
    fn bulk_put(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::BulkPutReq, sink: ::grpcio::UnarySink<super::indexrpcpb::BulkPutResp>);
    fn bulk_delete(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::BulkDeleteReq, sink: ::grpcio::UnarySink<super::indexrpcpb::BulkDeleteResp>);
    fn commit(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::CommitReq, sink: ::grpcio::UnarySink<super::indexrpcpb::CommitResp>);
    fn rollback(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::RollbackReq, sink: ::grpcio::UnarySink<super::indexrpcpb::RollbackResp>);
    fn merge(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::MergeReq, sink: ::grpcio::UnarySink<super::indexrpcpb::MergeResp>);
    fn search(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::SearchReq, sink: ::grpcio::UnarySink<super::indexrpcpb::SearchResp>);
    fn schema(&mut self, ctx: ::grpcio::RpcContext, req: super::indexrpcpb::SchemaReq, sink: ::grpcio::UnarySink<super::indexrpcpb::SchemaResp>);
}

pub fn create_index<S: Index + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_RAFT, move |ctx, req, resp| {
        instance.raft(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_RAFT_CONF_CHANGE, move |ctx, req, resp| {
        instance.raft_conf_change(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_PROBE, move |ctx, req, resp| {
        instance.probe(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_PEERS, move |ctx, req, resp| {
        instance.peers(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_METRICS, move |ctx, req, resp| {
        instance.metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_BULK_PUT, move |ctx, req, resp| {
        instance.bulk_put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_BULK_DELETE, move |ctx, req, resp| {
        instance.bulk_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_COMMIT, move |ctx, req, resp| {
        instance.commit(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_ROLLBACK, move |ctx, req, resp| {
        instance.rollback(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_MERGE, move |ctx, req, resp| {
        instance.merge(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_SEARCH, move |ctx, req, resp| {
        instance.search(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDEX_SCHEMA, move |ctx, req, resp| {
        instance.schema(ctx, req, resp)
    });
    builder.build()
}

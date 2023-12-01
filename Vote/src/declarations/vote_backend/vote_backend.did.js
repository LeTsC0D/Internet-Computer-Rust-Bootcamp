export const idlFactory = ({ IDL }) => {
  const CreateProposal = IDL.Record({
    'description' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const Proposal = IDL.Record({
    'reject' : IDL.Nat32,
    'owner' : IDL.Principal,
    'voted' : IDL.Vec(IDL.Principal),
    'pass' : IDL.Nat32,
    'approve' : IDL.Nat32,
    'description' : IDL.Text,
    'is_active' : IDL.Bool,
  });
  const VoteError = IDL.Variant({
    'AlreadyVoted' : IDL.Null,
    'ProposalIsNotActive' : IDL.Null,
    'AccessRejected' : IDL.Null,
    'NoSuchProposal' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : VoteError });
  const Choice = IDL.Variant({
    'Approve' : IDL.Null,
    'Pass' : IDL.Null,
    'Reject' : IDL.Null,
  });
  return IDL.Service({
    'create_proposal' : IDL.Func(
        [IDL.Nat64, CreateProposal],
        [IDL.Opt(Proposal)],
        [],
      ),
    'edit_proposal' : IDL.Func([IDL.Nat64, CreateProposal], [Result], []),
    'end_proposal' : IDL.Func([IDL.Nat64], [Result], []),
    'get_proposal' : IDL.Func([IDL.Nat64], [IDL.Opt(Proposal)], ['query']),
    'get_proposal_count' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_proposal_status' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Text)],
        ['query'],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'vote' : IDL.Func([IDL.Nat64, Choice], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };

export const idlFactory = ({ IDL }) => {
  const Exam = IDL.Record({
    'out_of' : IDL.Nat8,
    'curve' : IDL.Nat8,
    'course' : IDL.Text,
  });
  return IDL.Service({
    'get_exam' : IDL.Func([IDL.Nat64], [IDL.Opt(Exam)], ['query']),
    'get_participation' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Nat64)],
        ['query'],
      ),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'insert_exam' : IDL.Func([IDL.Nat64, Exam], [IDL.Opt(Exam)], []),
    'insert_participation' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Opt(IDL.Nat64)],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };

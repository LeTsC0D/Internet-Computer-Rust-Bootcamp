import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Exam { 'out_of' : number, 'curve' : number, 'course' : string }
export interface _SERVICE {
  'get_exam' : ActorMethod<[bigint], [] | [Exam]>,
  'get_participation' : ActorMethod<[bigint], [] | [bigint]>,
  'greet' : ActorMethod<[string], string>,
  'insert_exam' : ActorMethod<[bigint, Exam], [] | [Exam]>,
  'insert_participation' : ActorMethod<[bigint, bigint], [] | [bigint]>,
}

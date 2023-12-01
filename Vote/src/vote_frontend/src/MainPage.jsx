import { final_project_backend } from "../../../declarations/final_project_backend";
const getCurrentProposal = async (count) => {
  const getCurrentProposal = await final_project_backend.get_proposal(
      Number(count)
  );
  setCurrentProposal(getCurrentProposal);
};

await final_project_backend.get_proposal_count();
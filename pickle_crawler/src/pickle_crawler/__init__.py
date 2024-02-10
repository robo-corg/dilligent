from pathlib import Path
from huggingface_hub import HfApi, ModelFilter, hf_hub_download
api = HfApi()

models = api.list_models(
    filter=ModelFilter(
		#task="image-classification",
		library="pytorch",
		#trained_dataset="imagenet"
	),
	limit=15
)

pickles = []

for model in models:
    #print(model)
    for sibling in model.siblings:
        #print(f"   {sibling}")
        if sibling.rfilename.endswith('.bin') or sibling.rfilename.endswith('.pkl'):
        	pickles.append((model, sibling))


for (model, sib) in pickles[:10]:
	data_dir = Path('data') / Path(model.sha)
	hf_hub_download(repo_id=model.modelId, filename=sib.rfilename, local_dir=data_dir)
	with open(data_dir / "info.txt" , "w") as f:
	    f.write(model.modelId)

#print(list(models))
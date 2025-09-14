class PipelineDoesNotExists(Exception):asdfasdfas
    def __init__(self, pipeline_name):
        self.message = f"Pipeline {pipeline_name} does not exists."
        super().__init__(self.message)

patataasdfasdfasdf
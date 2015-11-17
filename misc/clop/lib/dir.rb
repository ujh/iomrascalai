require 'fileutils'

def dir(seed:, experiment:)
  caller_fn = caller.first.split(":").first
  output_dir = File.join(File.expand_path(File.dirname(caller_fn)), "experiments", experiment, seed.to_s)
  FileUtils.mkdir_p(output_dir)
  Dir.chdir(output_dir) do
    yield
  end
end

class DockerfileRepository

  def self.find(name)
    lookup[name] or raise "Could not find a dockerfile for '#{name}'"
  end

  private

  def self.lookup
    {
      empty: "spec/Dockerfile",
      initialized: "spec/dockerfiles/initialized",
    }
  end
end

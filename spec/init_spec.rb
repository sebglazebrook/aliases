CONTAINER_NAME = "aliases-test-container"
IMAGE_NAME = "aliases-test-image"

def build_container(dockerfile)
  puts '---- Building container'
  system("docker build --tag #{IMAGE_NAME} --file #{dockerfile} .")
end

def start_container(dockerfile)
  build_container(dockerfile)
  puts '---- Starting container'
  system("docker run -ti -v ${APP_ROOT}:/code -d --workdir /code --name #{CONTAINER_NAME} #{IMAGE_NAME} sh")
end

def kill_container
  puts '---- Killing container'
  system("docker rm --force #{CONTAINER_NAME}")
end

def docker_run(command, args, dockerfile)
  start_container(dockerfile)
  puts "---- Running command: #{command} #{args}"
  puts "docker exec -ti #{CONTAINER_NAME} #{command} #{args.join(" ")}"
  `docker exec -ti #{CONTAINER_NAME} #{command} #{args.join(" ")}`
end

def docker_diff
  `docker diff #{CONTAINER_NAME}`.split("\n")
end

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

describe "init command" do

  let(:dockerfile) { DockerfileRepository.find(:empty) }
  subject { docker_run(command, args, dockerfile) }

  after { kill_container }


  context "without any args" do

    let(:args) { [] }

    context "given the directory is uninitialized" do

      let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases init'" }

      it "creates an aliases file" do
        subject
        expect(docker_diff.include?("A /tmp/.aliases")).to be true
      end
    end

    context "given the directory is initialized" do

      let(:command) { "./target/debug/aliases init" }

      it "leaves the file system untouched" do
        subject
        expect(docker_diff.include?("A /code/.aliases")).to be false
      end
    end

    context "given the file system isn't initialized" do

      let(:command) { "./target/debug/aliases init" }

      it "creates a aliases config file" do
        subject
        expect(docker_diff.include?("A /root/.aliases_cfg")).to be true
      end
    end

    context "given the file system IS already initialized" do

      let(:dockerfile) { DockerfileRepository.find(:initialized) }

      let(:command) { "./target/debug/aliases init" }

      it "leaves the file system untouched" do
        subject
        expect(docker_diff.include?("A /root/.aliases_cfg")).to be false
      end
    end
  end

  describe "--global" do

    let(:args) { ["--global"] }
    let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases init --global'" }

    it "outputs the global system config users need to run to enable aliases" do
      expect(subject).to eq "export PATH=\"${HOME}/.aliases.d/shims:${PATH}\"\r\naliases rehash\r\n"
    end
  end

  describe "--user" do

    context "when given a username" do

      it "creates an alias file for that user"
    end
  end
end

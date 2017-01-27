CONTAINER_NAME = "aliases-test-container"
IMAGE_NAME = "aliases-test-image"

def build_container
  puts '---- Building container'
  system("docker build --tag #{IMAGE_NAME} --file spec/Dockerfile .")
end

def start_container
  build_container
  puts '---- Starting container'
  system("docker run -ti -v ${APP_ROOT}:/code -d --workdir /code --name #{CONTAINER_NAME} #{IMAGE_NAME} sh")
end

def kill_container
  puts '---- Killing container'
  system("docker rm --force #{CONTAINER_NAME}")
end

def docker_run(command, args)
  puts "---- Running command: #{command} #{args}"
  puts "docker exec -ti #{CONTAINER_NAME} #{command} #{args.join(" ")}"
  system("docker exec -ti #{CONTAINER_NAME} #{command} #{args.join(" ")}")
end

def docker_diff
  `docker diff #{CONTAINER_NAME}`.split("\n")
end

describe "init command" do

  subject { docker_run(command, args) }

  before { start_container }
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

    #context "given the file system IS already initialized" do

      #it "leaves the file system untouched" do
      #end
    #end
  end

  #describe "--global" do
  #end

  #describe "--user" do
  #end

end

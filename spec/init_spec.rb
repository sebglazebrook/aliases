describe "`init` command" do

  let(:dockerfile) { DockerfileRepository.find(:empty) }
  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "without any args" do

    let(:args) { [] }

    context "given the directory is uninitialized" do

      let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases init'" }

      it "creates an aliases file" do
        expect(subject.diff.include?("A /tmp/.aliases")).to be true
      end
    end

    context "given the directory is initialized" do

      let(:command) { "./target/debug/aliases init" }

      it "leaves the file system untouched" do
        expect(subject.diff.include?("A /code/.aliases")).to be false
      end
    end

    context "given the file system isn't initialized" do

      let(:command) { "./target/debug/aliases init" }

      it "creates a aliases config file" do
        expect(subject.diff.include?("A /root/.aliases_cfg")).to be true
      end
    end

    context "given the file system IS already initialized" do

      let(:dockerfile) { DockerfileRepository.find(:initialized) }

      let(:command) { "./target/debug/aliases init" }

      it "leaves the file system untouched" do
        expect(subject.diff.include?("A /root/.aliases_cfg")).to be false
      end
    end

    context "given the directory is a symlink" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'mkdir -p /tmp/original'")
        docker_command.query("bash -c 'cd /tmp/ && ln -s original linked'")
      end

    let(:command ) { "bash -c 'cd /tmp/original && /code/target/debug/aliases init'" }

      it "adds the sourced dir not the linked dir" do
        subject
        expect(docker_command.query('/code/target/debug/aliases directories')).to match("/tmp/original")
        expect(docker_command.query('/code/target/debug/aliases directories')).to_not match("/tmp/linked")
      end
    end
  end

  describe "--global" do

    let(:args) { ["--global"] }
    let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases init --global'" }
    let(:dockerfile) { DockerfileRepository.find(:initialized) }

    it "outputs the global system config users need to run to enable aliases" do
      expect(subject.output).to eq "export PATH=\"${HOME}/.aliases.d/shims:${PATH}\"\r\naliases rehash\r\n"
    end

    it "doesn't change the filesystem" do
      expect(subject.diff.empty?).to be true
    end
  end

  describe "--user" do

    context "when given a username" do

      let(:args) { ["--user", "superman"]  }
      let(:dockerfile) { DockerfileRepository.find(:initialized) }
      let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases init --user superman'" }

      it "creates an alias file for that user" do
        expect(subject.diff.include?("A /tmp/.aliases-superman")).to eq true
      end
    end
  end
end

describe "`add` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "on an initialized file system" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }

    context "without optional args" do

      let(:args) { [] }

      let(:command ) { "bash -c 'cd /tmp && /code/target/debug/aliases add foo \"hello world\"'" }

      it "creates the alias in the current directory" do
        subject
        expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list --local'")).to match("foo")
      end

      context "when the current directory is not initialized" do

        let(:command ) { "bash -c 'cd /tmp && mkdir new-uninitialized-dir && cd /tmp/new-uninitialized-dir && /code/target/debug/aliases add foo bar'" }

        it "initializes the directory" do
          subject
          expect(docker_command.query("bash -c 'aliases directories'")).to match("/tmp/new-uninitialized-dir")
        end
      end

      it "makes the alias available for use" do
        subject
        expect(docker_command.query("bash -c 'source ~/.bashrc && cd /tmp && foo'")).to match(/hello world/) #TODO shouldn't have to source the rc file here
      end
    end

    context "with optional args" do

      describe "--global" do
      end

      describe "--directory" do
      end

      describe "--user" do
      end
    end
  end
end

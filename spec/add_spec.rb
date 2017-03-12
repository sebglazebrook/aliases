describe "`add` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "on an initialized file system" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }

    context "without optional args" do

      let(:args) { [] }

      let(:command ) { "bash -c 'cd /tmp && /code/target/debug/aliases add c cat'" }

      it "creates the alias in the current directory" do
        subject
        expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list --local'").include?("c")).to be true
      end

      context "when the current directory is not initialized" do

        let(:command ) { "bash -c 'cd /tmp && mkdir new-uninitialized-dir && cd /tmp/new-uninitialized-dir && /code/target/debug/aliases add c cat'" }

        it "initializes the directory" do
          subject
          expect(docker_command.query("bash -c 'aliases directories'").include?("/tmp/new-uninitialized-dir")).to be true
        end
      end

      it "rehashes to make the alias available"
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

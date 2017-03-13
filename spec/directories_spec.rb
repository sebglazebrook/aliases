describe "`directories` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "on an initialized file system" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }

    let(:args) { [] }

    context "when there are initialized directories" do
      let(:command ) { "bash -c 'cd /tmp && /code/target/debug/aliases init'" }

      it "lists the directory" do
        subject
        expect(docker_command.query("bash -c '/code/target/debug/aliases directories'").include?("/tmp")).to be true
      end
    end
  end
end

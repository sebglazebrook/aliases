describe "`remove` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "on an initialized file system" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }

    let(:args) { [] }


    context 'when the alias exist' do
      before :each do
        DockerCommand.new(add_command, args, dockerfile)
      end

      let(:add_command ) { "bash -c 'cd /tmp && /code/target/debug/aliases add foo \"hello world\"'" }

      let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases remove foo \"hello world\"'" }

      it "removes the alias from the current directory" do
        subject
        expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list --local'")).to_not match("foo")
      end
    end
  end
end

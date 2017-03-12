describe "`list` command" do

  context "when aliases has been initialized" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }
    let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
    let(:args) { [] }
    subject { docker_command.invoke }

    after { docker_command.kill }

    context "and there are aliases in the home and current directory" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases add c cat'")
        docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases add l ls'")
      end

      context "with NO args" do

        let(:command) { "bash -c 'cd /tmp && aliases list'" }

        it "lists aliases in the home directory" do
          expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list'")).to match(/c\s+cat/)
        end

        it "lists aliases in the local directory" do
          expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list'")).to match(/l\s+ls/)
        end

        context "when there are matching aliases in both directories" do

          before do
            docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases add foo home-bar'")
            docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases add foo local-bar'")
          end

          it "lists the one in the local directory" do
            expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list'")).to match(/foo\s+local-bar/)
            expect(docker_command.query("bash -c 'cd /tmp && /code/target/debug/aliases list'")).to_not match(/foo\s+home-bar/)
          end
        end
      end

      context "with `--global` arg" do

        it "only lists the aliases in the home dir"
      end

      context "with `--local` arg" do

        it "only list the aliases in the local directory"
      end

      context "with `--directory` arg" do

        it "only lists the aliases in the given directory"
      end

      context "with `--name` arg" do

        it "only lists the aliases that matches the given name exactly"
      end
    end
  end
end

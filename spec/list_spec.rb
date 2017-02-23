describe "`list` command" do

  context "when aliases has been initialized" do

    let(:dockerfile) { DockerfileRepository.find(:initialized) }
    let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
    subject { docker_command.invoke }

    context "and there are aliases in the home and current directory" do

      before do
        # create a global and local alias
      end

      context "with NO args" do

        let(:command) { "bash -c 'cd /tmp && aliases list'" }

        it "lists aliases in the home directory" do
        end

        it "lists aliases in the local directory" do
        end

        context "when there are matching aliases in both directories" do

          before do
            # create a global and local alias that matches
          end

          it "lists the one in the local directory" do
          end
        end
      end

      context "with `--global` arg" do

        it "only lists the aliases in the home dir" do
        end
      end

      context "with `--local` arg" do

        it "only list the aliases in the local directory" do
        end
      end

      context "with `--directory` arg" do

        it "only lists the aliases in the given directory" do
        end
      end

      context "with `--name` arg" do

        it "only lists the aliases that matches the given name exactly" do
        end
      end
    end
  end
end

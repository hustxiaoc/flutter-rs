app_name=$1;
export app_name=$app_name;
project_dir="examples/$app_name";
if [ ! -x "$project_dir" ]; then
  echo "$project_dir not exist";
  exit 1;
fi
cd $project_dir;
curdir=`pwd`;
file="$curdir/pubspec.yaml";

if [ ! -f "$file" ]; then
  echo "project is not a flutter project";
  exit 1;
fi
cargo flutter run

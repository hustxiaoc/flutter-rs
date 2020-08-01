project_dir="examples/$1";
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

use std::fs::File;
use std::fs::read_dir;
use std::path::Path;
use std::io::Error;
use std::io::Write;
use std::ffi::OsStr;
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};


#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Actor {
    id: u64,
    name: String,
    cast_id: u64,
    credit_id: String,
    character: String,
    profile_path: Option<String>
}


#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Movie {
    foldername: Option<String>,
    title: Option<String>,
    year: Option<String>,
    quality: Option<String>,
    tmdbid: Option<String>,
    imdbid: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    tagline: Option<String>,
    genres: Vec<String>,
    cast: Vec<Actor>
}

/*
-------------------------
*/

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Details {
    imdbid: Option<String>,
    poster_path: String,
    backdrop_path: Option<String>,
    tagline: String,
    genres: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OldMovie {
    title: String,
    year: String,
    quality: String,
    tmdbid: String,
    details: Details,
    cast: Vec<Actor>
}


fn list_of_movies_unorganized(filepath: &Path) -> Result<Vec<Movie>, Error> {
    let mut movies: Vec<Movie> = Vec::new();
    
    for entry in read_dir(filepath)? {
        let path = entry?.path();
        let filename_ostr = path.file_name().unwrap();
        let filename = String::from(filename_ostr.to_str().unwrap());
        let movie = Movie {
            foldername: Some(filename),
            title: None,
            year: None,
            quality: None,
            tmdbid: None,
            imdbid: None,
            poster_path: None,
            backdrop_path: None,
            tagline: None,
            genres: Vec::new(),
            cast: Vec::new()
        };
        movies.push(movie);
    }

    return Ok(movies);
}

fn list_of_movies_organized(folderpath: &Path) -> Result<Vec<Movie>, Error> {
    let mut movies: Vec<Movie> = Vec::new();

    for year_entry in read_dir(folderpath)? {
        let year_path = year_entry?.path();
        let year_string = String::from(year_path.file_name().unwrap().to_str().unwrap());

        for movie_entry in read_dir(year_path)? {
            let movie_path = movie_entry?.path();
            let movie_string = String::from(movie_path.file_name().unwrap().to_str().unwrap());
            let movie = Movie {
                foldername: Some(movie_string),
                title: None,
                year: Some(year_string.to_string()),
                quality: None,
                tmdbid: None,
                imdbid: None,
                poster_path: None,
                backdrop_path: None,
                tagline: None,
                genres: Vec::new(),
                cast: Vec::new()
            };
            movies.push(movie);
        }
    }

    return Ok(movies);
}

fn write_movies(movies: Vec<Movie>, filepath: &Path) {
    let movies_json = serde_json::to_string_pretty(&movies).unwrap();
    let mut file = File::create(filepath).expect("File cannot be created");
    file.write_all(movies_json.as_bytes())
        .expect("Failed to write");
}

fn read_movies(filepath: &Path) -> Vec<Movie> {
    let file = File::open(filepath).expect("file not found");
    let movies: Vec<Movie> = serde_json::from_reader(file)
        .expect("error reading file");
    return movies;
}

fn read_oldmovie_from_file(filepath: &Path) -> OldMovie {
    let file = File::open(filepath).expect("file not found");
    let movie: OldMovie = serde_json::from_reader(file)
        .expect("error reading file");
    return movie;
}

fn read_movies_from_individual_files(folderpath: &Path) -> Result<Vec<Movie>, Error> {
    let mut movies: Vec<Movie> = Vec::new();

    for year_entry in read_dir(folderpath)? {
        let year_path = year_entry?.path();

        for movie_entry in read_dir(year_path)? {
            let movie_path = movie_entry?.path();

            for file_entry in read_dir(movie_path)? {
                let file_entry_path = file_entry?.path();
                let filename = file_entry_path.file_name().unwrap().to_str().unwrap();
                
                if filename == "metadata-file.json" {
                    let oldmovie = read_oldmovie_from_file(&file_entry_path);
                    let movie = Movie {
                        foldername: None,
                        title: Some(oldmovie.title.to_string()),
                        year: Some(oldmovie.year.to_string()),
                        quality: Some(oldmovie.quality.to_string()),
                        tmdbid: Some(oldmovie.tmdbid.to_string()),
                        imdbid: oldmovie.details.imdbid,
                        poster_path: Some(oldmovie.details.poster_path.to_string()),
                        backdrop_path: oldmovie.details.backdrop_path,
                        tagline: Some(oldmovie.details.tagline.to_string()),
                        genres: oldmovie.details.genres,
                        cast: oldmovie.cast
                    };
                    movies.push(movie);
                }
            }
        }
    }

    return Ok(movies);
}

fn read_extensions(filepath: &Path) -> Result<Vec<String>, Error> {
    let mut extensions: Vec<String> = Vec::new();

    for entry in read_dir(filepath)? {
        let movie_path = entry?.path();

        if movie_path.is_dir() {
            for file_entry in read_dir(movie_path)? {
                let file_entry_path = file_entry?.path();
                if file_entry_path.is_file() {
                    let extension = file_entry_path
                        .extension()
                        .and_then(OsStr::to_str);

                    match extension {
                        Some(ext) => {
                            let extension_str = String::from(ext);
                            if !extensions.contains(&extension_str) {
                                extensions.push(extension_str);
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    return Ok(extensions);
}

fn read_all_files_unordered(filepath: &Path) -> Result<Vec<String>, Error> {
    let mut filenames: Vec<String> = Vec::new();

    for entry in read_dir(filepath)? {
        let movie_path = entry?.path();

        if movie_path.is_dir() {
            for file_entry in read_dir(movie_path)? {
                let file_entry_path = file_entry?.path();

                if file_entry_path.is_file() {
                    let filename_string = String::from(file_entry_path.file_name().unwrap().to_str().unwrap());
                    filenames.push(filename_string);
                }
            }
        }
    }

    return Ok(filenames);
}

fn read_all_files_ordered(filepath: &Path) -> Result<Vec<String>, Error> {
    let mut filenames: Vec<String> = Vec::new();

    for entry in read_dir(filepath)? {
        let year_path = entry?.path();

        for movie_entry in read_dir(year_path)? {
            let movie_entry_path = movie_entry?.path();

            for file_entry in read_dir(movie_entry_path)? {
                let file_entry_path = file_entry?.path();
                if file_entry_path.is_file() {
                    let filename_string = String::from(file_entry_path.file_name().unwrap().to_str().unwrap());
                    filenames.push(filename_string);
                }
            }
        }
    }
    println!("{}", filenames.len());

    return Ok(filenames);
}

fn count_all_movies_ordered(filepath: &Path) -> Result<u64, Error> {
    let mut count: u64 = 0;

    for entry in read_dir(filepath)? {
        let year_path = entry?.path();

        for movie_entry in read_dir(year_path)? {
            count += 1;
        }
    }

    return Ok(count);
}

fn count_all_movies_unordered(filepath: &Path) -> Result<u64, Error> {
    let mut count: u64 = 0;

    for entry in read_dir(filepath)? {
        count += 1;
    }

    return Ok(count);
}

fn print_unique_movies(filepath1: &Path, filepath2: &Path) {
    let files1 = read_all_files_ordered(filepath1).unwrap();
    let files2 = read_all_files_unordered(filepath2).unwrap();

    for file in files2 {
        if !files1.contains(&file) {
            println!("{}", file);
        }
    }
}

fn write_unique_files(filepath1: &Path, filepath2: &Path) {
    let files1 = read_all_files_ordered(filepath1).unwrap();
    let files2 = read_all_files_unordered(filepath2).unwrap();
    let mut unique_files: Vec<String> = Vec::new();

    for file in files2 {
        if !files1.contains(&file) {
            unique_files.push(file.to_string());
        }
    }

    let unique_files_json = serde_json::to_string_pretty(&unique_files).unwrap();
    let mut file = File::create(Path::new("/home/ruslan/Code/rust/movie-organizer-cli/files.json"))
        .expect("cannot create files");
    file.write_all(unique_files_json.as_bytes()).expect("files could not be written");
}

fn generate_movies_file() {
    let filepath = Path::new("/run/media/ruslan/MultiMedia/Ruslan/Movies");
    let output_filepath = Path::new("/home/ruslan/Code/rust/movie-organizer-cli/multimedia-movies.json");
    let movies = list_of_movies_unorganized(&filepath).unwrap();
    println!("{}", movies.len());
    write_movies(movies, output_filepath);
}

fn generate_file_from_ordered_folder(folderpath: &Path, filepath: &Path) {
    let movies = list_of_movies_organized(folderpath).unwrap();
    write_movies(movies, filepath);
}

fn print_datahome_movie_count() {
    let ordered_path = Path::new("/run/media/ruslan/DataHome/Ruslan/MoviesByYear");
    println!("{} datahome movies", count_all_movies_ordered(&ordered_path).unwrap());
}

fn print_multimedia_movie_count() {
    let ordered_path = Path::new("/run/media/ruslan/MultiMedia/Movies");
    println!("{} multimedia movies", count_all_movies_ordered(&ordered_path).unwrap());
}

fn print_all_movie_counts() {
    print_datahome_movie_count();
    print_multimedia_movie_count();
}

fn collect_metafiles_into_one(folderpath: &Path, filepath: &Path) {
    let movies = read_movies_from_individual_files(folderpath).unwrap();
    write_movies(movies, filepath);
}

fn print_movies_for_year(filepath: &Path, year: String) {
    let movies = read_movies(filepath);

    for movie in movies {
        if movie.year == Some(year.to_string()) {
            println!("{}", movie.title.unwrap());
        }
    }
}

fn print_movie_year_for_title(filepath: &Path, title: String) {
    let movies = read_movies(filepath);
    
    for movie in movies {
        if movie.title == Some(title.to_string()) {
            println!("title: {}", movie.title.unwrap());
            println!("year: {}", movie.year.unwrap());
        }
    }
}


#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Counts movies
    Count { argument: String },
    /// Generate JSON file from ordered moviefolder
    GenerateOrdered { folderpath_string: String, filepath_string: String },
    /// Collect all metafiles into one file
    Collect { folderpath_string: String, filepath_string: String },
    /// Print movie titles for a given year
    FindYear { filepath_string: String, year: String },
    /// Print movie year for a given title
    FindMovie { filepath_string: String, title: String }
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Count { argument } => {
            match argument.as_str() {
                "all" => { print_all_movie_counts(); },
                "datahome" => { print_datahome_movie_count(); },
                "multimedia" => { print_multimedia_movie_count(); },
                _ => { println!("No argument given"); }
            }
        },
        Commands::GenerateOrdered { folderpath_string, filepath_string } => {
            let folderpath = Path::new(folderpath_string.as_str());
            let filepath = Path::new(filepath_string.as_str());
            generate_file_from_ordered_folder(&folderpath, &filepath);
        },
        Commands::Collect { folderpath_string, filepath_string } => {
            let folderpath = Path::new(folderpath_string.as_str());
            let filepath = Path::new(filepath_string.as_str());
            collect_metafiles_into_one(&folderpath, &filepath);
        },
        Commands::FindYear { filepath_string, year } => {
            let filepath = Path::new(filepath_string.as_str());
            print_movies_for_year(&filepath, year.to_string());
        },
        Commands::FindMovie { filepath_string, title } => {
            let filepath = Path::new(filepath_string.as_str());
            print_movie_year_for_title(&filepath, title.to_string());
        }
    }
}

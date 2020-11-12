using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.IO;

namespace first
{
    public partial class FMain : Form
    {
        public FMain()
        {
            InitializeComponent();
        }

        private void Bexit_Click(object sender, EventArgs e)
        {
            //Application.Exit();
            System.Environment.Exit(0);
        }

        private void bedit_Click(object sender, EventArgs e)
        {
            this.tboxfile.ReadOnly = !this.tboxfile.ReadOnly;
            if (this.tboxfile.ReadOnly){
                this.tboxfile.BackColor = Color.Black;
                this.tboxfile.ForeColor= Color.White;
            }
            else
            {
                this.tboxfile.BackColor = Color.White;
                this.tboxfile.ForeColor = Color.Black;
            }
                
        }

        private void FMain_Load(object sender, EventArgs e)
        {
            string[] strs = { "ab", "cd", "ef", "gh" };
            string path = "config.toml";
            string ss = Environment.CurrentDirectory;
            this.textdir.Text = ss;
            if (File.Exists(path))
            {
                strs = File.ReadAllLines(path);
                //        string s = String.Join("\r\n", strs);
                //       Text1.Text = s;
            }

            string s = String.Join("\r\n", strs);
            this.tboxfile.Text = s;
            this.tboxfile.ReadOnly = true;


        }

        private void bselect_Click(object sender, EventArgs e)
        {
            //this.folderBrowserDialog1 = new System.Windows.Forms.FolderBrowserDialog();
            var fs = new FolderBrowserDialog();
            fs.ShowDialog();
            this.textdir.Text = fs.SelectedPath;
        }
    }
}

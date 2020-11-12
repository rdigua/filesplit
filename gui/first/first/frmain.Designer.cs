
namespace first
{
    partial class FMain
    {
        /// <summary>
        /// 必需的设计器变量。
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 清理所有正在使用的资源。
        /// </summary>
        /// <param name="disposing">如果应释放托管资源，为 true；否则为 false。</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows 窗体设计器生成的代码

        /// <summary>
        /// 设计器支持所需的方法 - 不要修改
        /// 使用代码编辑器修改此方法的内容。
        /// </summary>
        private void InitializeComponent()
        {
            this.Bexit = new System.Windows.Forms.Button();
            this.tboxfile = new System.Windows.Forms.TextBox();
            this.bedit = new System.Windows.Forms.Button();
            this.textdir = new System.Windows.Forms.TextBox();
            this.bselect = new System.Windows.Forms.Button();
            this.l_title = new System.Windows.Forms.Label();
            this.l_dim = new System.Windows.Forms.Label();
            this.l_left = new System.Windows.Forms.Label();
            this.l_right = new System.Windows.Forms.Label();
            this.lmethod = new System.Windows.Forms.Label();
            this.loutput = new System.Windows.Forms.Label();
            this.l_dir = new System.Windows.Forms.Label();
            this.lsplitting = new System.Windows.Forms.Label();
            this.label1 = new System.Windows.Forms.Label();
            this.SuspendLayout();
            // 
            // Bexit
            // 
            this.Bexit.Location = new System.Drawing.Point(713, 415);
            this.Bexit.Name = "Bexit";
            this.Bexit.Size = new System.Drawing.Size(75, 23);
            this.Bexit.TabIndex = 0;
            this.Bexit.Text = "Exit";
            this.Bexit.UseVisualStyleBackColor = true;
            this.Bexit.Click += new System.EventHandler(this.Bexit_Click);
            // 
            // tboxfile
            // 
            this.tboxfile.Location = new System.Drawing.Point(3, 2);
            this.tboxfile.Multiline = true;
            this.tboxfile.Name = "tboxfile";
            this.tboxfile.Size = new System.Drawing.Size(415, 436);
            this.tboxfile.TabIndex = 1;
            // 
            // bedit
            // 
            this.bedit.Location = new System.Drawing.Point(713, 386);
            this.bedit.Name = "bedit";
            this.bedit.Size = new System.Drawing.Size(75, 23);
            this.bedit.TabIndex = 2;
            this.bedit.Text = "Edit";
            this.bedit.UseVisualStyleBackColor = true;
            this.bedit.Click += new System.EventHandler(this.bedit_Click);
            // 
            // textdir
            // 
            this.textdir.Location = new System.Drawing.Point(530, 170);
            this.textdir.Name = "textdir";
            this.textdir.Size = new System.Drawing.Size(258, 21);
            this.textdir.TabIndex = 3;
            // 
            // bselect
            // 
            this.bselect.Location = new System.Drawing.Point(713, 357);
            this.bselect.Name = "bselect";
            this.bselect.Size = new System.Drawing.Size(75, 23);
            this.bselect.TabIndex = 4;
            this.bselect.Text = "SelectDir";
            this.bselect.UseVisualStyleBackColor = true;
            this.bselect.Click += new System.EventHandler(this.bselect_Click);
            // 
            // l_title
            // 
            this.l_title.AutoSize = true;
            this.l_title.ForeColor = System.Drawing.SystemColors.Info;
            this.l_title.Location = new System.Drawing.Point(594, 9);
            this.l_title.Name = "l_title";
            this.l_title.Size = new System.Drawing.Size(41, 12);
            this.l_title.TabIndex = 5;
            this.l_title.Text = "label1";
            // 
            // l_dim
            // 
            this.l_dim.AutoSize = true;
            this.l_dim.ForeColor = System.Drawing.SystemColors.ButtonFace;
            this.l_dim.Location = new System.Drawing.Point(442, 59);
            this.l_dim.Name = "l_dim";
            this.l_dim.Size = new System.Drawing.Size(35, 12);
            this.l_dim.TabIndex = 6;
            this.l_dim.Text = "[Dim]";
            // 
            // l_left
            // 
            this.l_left.AutoSize = true;
            this.l_left.BackColor = System.Drawing.SystemColors.Control;
            this.l_left.Location = new System.Drawing.Point(455, 91);
            this.l_left.Name = "l_left";
            this.l_left.Size = new System.Drawing.Size(29, 12);
            this.l_left.TabIndex = 7;
            this.l_left.Text = "left";
            // 
            // l_right
            // 
            this.l_right.AutoSize = true;
            this.l_right.BackColor = System.Drawing.SystemColors.Control;
            this.l_right.Location = new System.Drawing.Point(455, 122);
            this.l_right.Name = "l_right";
            this.l_right.Size = new System.Drawing.Size(35, 12);
            this.l_right.TabIndex = 8;
            this.l_right.Text = "right";
            // 
            // lmethod
            // 
            this.lmethod.AutoSize = true;
            this.lmethod.BackColor = System.Drawing.SystemColors.Control;
            this.lmethod.Location = new System.Drawing.Point(461, 237);
            this.lmethod.Name = "lmethod";
            this.lmethod.Size = new System.Drawing.Size(41, 12);
            this.lmethod.TabIndex = 11;
            this.lmethod.Text = "method";
            // 
            // loutput
            // 
            this.loutput.AutoSize = true;
            this.loutput.BackColor = System.Drawing.SystemColors.Control;
            this.loutput.Location = new System.Drawing.Point(461, 179);
            this.loutput.Name = "loutput";
            this.loutput.Size = new System.Drawing.Size(41, 12);
            this.loutput.TabIndex = 10;
            this.loutput.Text = "Output";
            // 
            // l_dir
            // 
            this.l_dir.AutoSize = true;
            this.l_dir.ForeColor = System.Drawing.SystemColors.ButtonFace;
            this.l_dir.Location = new System.Drawing.Point(448, 147);
            this.l_dir.Name = "l_dir";
            this.l_dir.Size = new System.Drawing.Size(71, 12);
            this.l_dir.TabIndex = 9;
            this.l_dir.Text = "[Directory]";
            // 
            // lsplitting
            // 
            this.lsplitting.AutoSize = true;
            this.lsplitting.ForeColor = System.Drawing.SystemColors.ButtonFace;
            this.lsplitting.Location = new System.Drawing.Point(448, 206);
            this.lsplitting.Name = "lsplitting";
            this.lsplitting.Size = new System.Drawing.Size(71, 12);
            this.lsplitting.TabIndex = 12;
            this.lsplitting.Text = "[splitting]";
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.BackColor = System.Drawing.SystemColors.Control;
            this.label1.Location = new System.Drawing.Point(461, 264);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(23, 12);
            this.label1.TabIndex = 13;
            this.label1.Text = "len";
            // 
            // FMain
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.SystemColors.ActiveCaptionText;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.label1);
            this.Controls.Add(this.lsplitting);
            this.Controls.Add(this.lmethod);
            this.Controls.Add(this.loutput);
            this.Controls.Add(this.l_dir);
            this.Controls.Add(this.l_right);
            this.Controls.Add(this.l_left);
            this.Controls.Add(this.l_dim);
            this.Controls.Add(this.l_title);
            this.Controls.Add(this.bselect);
            this.Controls.Add(this.textdir);
            this.Controls.Add(this.bedit);
            this.Controls.Add(this.tboxfile);
            this.Controls.Add(this.Bexit);
            this.Name = "FMain";
            this.Text = "SplitFile";
            this.Load += new System.EventHandler(this.FMain_Load);
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Button Bexit;
        private System.Windows.Forms.TextBox tboxfile;
        private System.Windows.Forms.Button bedit;
        private System.Windows.Forms.TextBox textdir;
        private System.Windows.Forms.Button bselect;
        private System.Windows.Forms.Label l_title;
        private System.Windows.Forms.Label l_dim;
        private System.Windows.Forms.Label l_left;
        private System.Windows.Forms.Label l_right;
        private System.Windows.Forms.Label lmethod;
        private System.Windows.Forms.Label loutput;
        private System.Windows.Forms.Label l_dir;
        private System.Windows.Forms.Label lsplitting;
        private System.Windows.Forms.Label label1;
    }
}

